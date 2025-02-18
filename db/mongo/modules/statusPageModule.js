import StatusPage from "../../models/StatusPage.js";
import { NormalizeData } from "../../../utils/dataUtils.js";
import ServiceRegistry from "../../../service/serviceRegistry.js";
import StringService from "../../../service/stringService.js";

const SERVICE_NAME = "statusPageModule";

const createStatusPage = async (statusPageData, image) => {
	const stringService = ServiceRegistry.get(StringService.SERVICE_NAME);

	try {
		const statusPage = new StatusPage({ ...statusPageData });
		if (image) {
			statusPage.logo = {
				data: image.buffer,
				contentType: image.mimetype,
			};
		}
		await statusPage.save();
		return statusPage;
	} catch (error) {
		if (error?.code === 11000) {
			// Handle duplicate URL errors
			error.status = 400;
			error.message = stringService.statusPageUrlNotUnique;
		}
		error.service = SERVICE_NAME;
		error.method = "createStatusPage";
		throw error;
	}
};

const updateStatusPage = async (statusPageData, image) => {
	try {
		if (image) {
			statusPageData.logo = {
				data: image.buffer,
				contentType: image.mimetype,
			};
		}
		const statusPage = await StatusPage.findOneAndUpdate(
			{ url: statusPageData.url },
			statusPageData,
			{
				new: true,
			}
		);

		return statusPage;
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "updateStatusPage";
		throw error;
	}
};

const getStatusPageByUrl = async (url, type) => {
	try {
		if (type === "distributed") {
			const statusPage = await StatusPage.aggregate([{ $match: { url } }]);
			return statusPage[0];
		} else {
			return getStatusPage(url);
		}
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "getStatusPageByUrl";
		throw error;
	}
};

const getStatusPagesByTeamId = async (teamId) => {
	const stringService = ServiceRegistry.get(StringService.SERVICE_NAME);

	try {
		const statusPages = await StatusPage.find({ teamId });
		return statusPages;
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "getStatusPagesByTeamId";
		throw error;
	}
};

const getStatusPage = async (url) => {
	const stringService = ServiceRegistry.get(StringService.SERVICE_NAME);

	try {
		const statusPageQuery = await StatusPage.aggregate([
			{ $match: { url: url } },
			{
				$set: {
					originalMonitors: "$monitors",
				},
			},
			{
				$lookup: {
					from: "monitors",
					localField: "monitors",
					foreignField: "_id",
					as: "monitors",
				},
			},
			{
				$unwind: "$monitors",
			},
			{
				$lookup: {
					from: "checks",
					let: { monitorId: "$monitors._id" },
					pipeline: [
						{
							$match: {
								$expr: { $eq: ["$monitorId", "$$monitorId"] },
							},
						},
						{ $sort: { createdAt: -1 } },
						{ $limit: 25 },
					],
					as: "monitors.checks",
				},
			},
			{
				$addFields: {
					"monitors.orderIndex": {
						$indexOfArray: ["$originalMonitors", "$monitors._id"],
					},
				},
			},
			{
				$group: {
					_id: "$_id",
					statusPage: { $first: "$$ROOT" },
					monitors: { $push: "$monitors" },
				},
			},
			{
				$project: {
					statusPage: {
						_id: 1,
						color: 1,
						companyName: 1,
						isPublished: 1,
						logo: 1,
						originalMonitors: 1,
						showCharts: 1,
						showUptimePercentage: 1,
						timezone: 1,
						url: 1,
					},
					monitors: {
						$sortArray: {
							input: "$monitors",
							sortBy: { orderIndex: 1 },
						},
					},
				},
			},
		]);
		if (!statusPageQuery.length) {
			const error = new Error(stringService.statusPageNotFound);
			error.status = 404;
			throw error;
		}

		const { statusPage, monitors } = statusPageQuery[0];

		const normalizedMonitors = monitors.map((monitor) => {
			return {
				...monitor,
				checks: NormalizeData(monitor.checks, 10, 100),
			};
		});

		return { statusPage, monitors: normalizedMonitors };
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "getStatusPageByUrl";
		throw error;
	}
};

const deleteStatusPage = async (url) => {
	try {
		await StatusPage.deleteOne({ url });
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "deleteStatusPage";
		throw error;
	}
};

const deleteStatusPagesByMonitorId = async (monitorId) => {
	try {
		await StatusPage.deleteMany({ monitors: { $in: [monitorId] } });
	} catch (error) {
		error.service = SERVICE_NAME;
		error.method = "deleteStatusPageByMonitorId";
		throw error;
	}
};

export {
	createStatusPage,
	updateStatusPage,
	getStatusPagesByTeamId,
	getStatusPage,
	getStatusPageByUrl,
	deleteStatusPage,
	deleteStatusPagesByMonitorId,
};

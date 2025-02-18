const responseHandler = (req, res, next) => {
	res.success = ({ status = 200, msg = "OK", data = null }) => {
		return res.status(status).json({
			success: true,
			msg: msg,
			data: data,
		});
	};

	res.error = ({ status = 500, msg = "Internal server error", data = null }) => {
		return res.status(status).json({
			success: false,
			msg,
			data,
		});
	};
	next();
};

export { responseHandler };

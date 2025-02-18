import { Router } from "express";
import { verifyOwnership } from "../middleware/verifyOwnership.js";
import { isAllowed } from "../middleware/isAllowed.js";
import Monitor from "../db/models/Monitor.js";

class CheckRoutes {
	constructor(checkController) {
		this.router = Router();
		this.checkController = checkController;
		this.initRoutes();
	}

	initRoutes() {
		this.router.get("/:monitorId", this.checkController.getChecksByMonitor);
		this.router.post(
			"/:monitorId",
			verifyOwnership(Monitor, "monitorId"),
			this.checkController.createCheck
		);
		this.router.delete(
			"/:monitorId",
			verifyOwnership(Monitor, "monitorId"),
			this.checkController.deleteChecks
		);

		this.router.get("/team/:teamId", this.checkController.getChecksByTeam);

		this.router.delete(
			"/team/:teamId",
			isAllowed(["admin", "superadmin"]),
			this.checkController.deleteChecksByTeamId
		);

		this.router.put(
			"/team/ttl",
			isAllowed(["admin", "superadmin"]),
			this.checkController.updateChecksTTL
		);
	}

	getRouter() {
		return this.router;
	}
}

export default CheckRoutes;

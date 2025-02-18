import mongoose from "mongoose";

const BaseCheckSchema = mongoose.Schema({
	/**
	 * Reference to the associated Monitor document.
	 *
	 * @type {mongoose.Schema.Types.ObjectId}
	 */
	monitorId: {
		type: mongoose.Schema.Types.ObjectId,
		ref: "Monitor",
		immutable: true,
		index: true,
	},

	teamId: {
		type: mongoose.Schema.Types.ObjectId,
		ref: "Team",
		immutable: true,
		index: true,
	},
	/**
	 * Status of the check (true for up, false for down).
	 *
	 * @type {Boolean}
	 */
	status: {
		type: Boolean,
		index: true,
	},
	/**
	 * Response time of the check in milliseconds.
	 *
	 * @type {Number}
	 */
	responseTime: {
		type: Number,
	},
	/**
	 * HTTP status code received during the check.
	 *
	 * @type {Number}
	 */
	statusCode: {
		type: Number,
		index: true,
	},
	/**
	 * Message or description of the check result.
	 *
	 * @type {String}
	 */
	message: {
		type: String,
	},
	/**
	 * Expiry date of the check, auto-calculated to expire after 30 days.
	 *
	 * @type {Date}
	 */

	expiry: {
		type: Date,
		default: Date.now,
		expires: 60 * 60 * 24 * 30, // 30 days
	},
});

/**
 * Check Schema for MongoDB collection.
 *
 * Represents a check associated with a monitor, storing information
 * about the status and response of a particular check event.
 */
const CheckSchema = mongoose.Schema({ ...BaseCheckSchema.obj }, { timestamps: true });
CheckSchema.index({ createdAt: 1 });
CheckSchema.index({ monitorId: 1, createdAt: 1 });
CheckSchema.index({ monitorId: 1, createdAt: -1 });
CheckSchema.index({ teamId: 1, createdAt: -1 });
CheckSchema.index({ teamId: 1 });

export default mongoose.model("Check", CheckSchema);
export { BaseCheckSchema };

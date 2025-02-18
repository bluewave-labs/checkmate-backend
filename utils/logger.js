import { createLogger, format, transports } from "winston";

class Logger {
	constructor() {
		const consoleFormat = format.printf(
			({ level, message, service, method, details, timestamp }) => {
				if (message instanceof Object) {
					message = JSON.stringify(message, null, 2);
				}

				if (details instanceof Object) {
					details = JSON.stringify(details, null, 2);
				}
				let msg = `${timestamp} ${level}:`;
				service && (msg += ` [${service}]`);
				method && (msg += `(${method})`);
				message && (msg += ` ${message}`);
				details && (msg += ` (details: ${details})`);
				return msg;
			}
		);

		this.logger = createLogger({
			level: "info",
			format: format.combine(format.timestamp()),
			transports: [
				new transports.Console({
					format: format.combine(
						format.colorize(),
						format.prettyPrint(),
						format.json(),
						consoleFormat
					),
				}),
				new transports.File({
					format: format.combine(format.json()),
					filename: "app.log",
				}),
			],
		});
	}
	/**
	 * Logs an informational message.
	 * @param {Object} config - The configuration object.
	 * @param {string} config.message - The message to log.
	 * @param {string} config.service - The service name.
	 * @param {string} config.method - The method name.
	 * @param {Object} config.details - Additional details.
	 */
	info(config) {
		this.logger.info(config.message, {
			service: config.service,
			method: config.method,
			details: config.details,
		});
	}

	/**
	 * Logs a warning message.
	 * @param {Object} config - The configuration object.
	 * @param {string} config.message - The message to log.
	 * @param {string} config.service - The service name.
	 * @param {string} config.method - The method name.
	 * @param {Object} config.details - Additional details.
	 */
	warn(config) {
		this.logger.warn(config.message, {
			service: config.service,
			method: config.method,
			details: config.details,
		});
	}

	/**
	 * Logs an error message.
	 * @param {Object} config - The configuration object.
	 * @param {string} config.message - The message to log.
	 * @param {string} config.service - The service name.
	 * @param {string} config.method - The method name.
	 * @param {Object} config.details - Additional details.
	 */
	error(config) {
		this.logger.error(config.message, {
			service: config.service,
			method: config.method,
			details: config.details,
			stack: config.stack,
		});
	}
}

const logger = new Logger();
export { Logger };

export default logger;

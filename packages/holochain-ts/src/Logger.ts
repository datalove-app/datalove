/* eslint-global LOG_LEVEL */

declare const LOG_LEVEL: string;

const DEBUG_PREFIX = `${App.Agent.String}: DEBUG ->>`;
const INFO_PREFIX = `${App.Agent.String}: INFO  ->>`;
const WARN_PREFIX = `${App.Agent.String}: WARN  ->>`;
const ERROR_PREFIX = `${App.Agent.String}: ERROR ->>`;

const NOOP = () => {};
const DEFAULT_LOG_LEVEL = 'OFF';
const LOG_LEVELS: { [logLevel: string]: number } = {
  DEBUG: 0,
  INFO: 1,
  WARN: 2,
  ERROR: 3,
  OFF: 4,
};

const getCurrentLogLevel = (): string => (
  Object.prototype.hasOwnProperty.call(LOG_LEVELS, LOG_LEVEL)
    ? LOG_LEVEL
    : DEFAULT_LOG_LEVEL
);

const shouldLog = (fnLogLevel: string) =>
  LOG_LEVELS[fnLogLevel] >= LOG_LEVELS[getCurrentLogLevel()];

const createLogger = (key: string, prefix: string) => (
  (getCurrentLogLevel() === 'OFF' || !shouldLog(key))
    ? NOOP
    : (logString: any): void => debug(`${prefix}: ${logString}`)
);

export default {
  debug: createLogger('DEBUG', DEBUG_PREFIX),
  info: createLogger('INFO', INFO_PREFIX),
  warn: createLogger('WARN', WARN_PREFIX),
  error: createLogger('ERROR', ERROR_PREFIX),
};

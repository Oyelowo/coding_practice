import { CastError } from "mongoose";

// Mongoose

export interface ValidatorError extends Error {
    name: 'ValidatorError';
    properties: {message: string, type?: string, path?: string, value?: any, reason?: any};
    kind: string;
    path: string;
    value: any;
    reason: any;

    formatMessage(msg: string | Function, properties: any): string;

    toString(): string;
  }

  export interface ValidationError extends Error {
    name: 'ValidationError';

    errors: {[path: string]: ValidatorError | CastError};

    /** Console.log helper */
    toString(): string;

    inspect(): object;

    toJSON(): object;

    addError(path: string, error: any): void;
  }

  export type MiddleWareValidationError = CastError | ValidationError;

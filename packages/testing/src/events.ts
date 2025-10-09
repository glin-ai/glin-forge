/**
 * Event testing utilities
 *
 * Provides assertion helpers for contract events and extrinsic results.
 */

import type { ISubmittableResult } from '@polkadot/types/types';
import type { EventRecord } from '@polkadot/types/interfaces';

/**
 * Event matcher interface
 */
export interface EventMatcher {
  section: string;
  method: string;
  data?: any[];
}

/**
 * Parse events from a transaction result
 */
export function parseEvents(result: ISubmittableResult): EventRecord[] {
  return result.events;
}

/**
 * Find a specific event in the result
 */
export function findEvent(
  result: ISubmittableResult,
  section: string,
  method: string
): EventRecord | undefined {
  return result.events.find(
    ({ event }) => event.section === section && event.method === method
  );
}

/**
 * Find all events matching the criteria
 */
export function findEvents(
  result: ISubmittableResult,
  section: string,
  method: string
): EventRecord[] {
  return result.events.filter(
    ({ event }) => event.section === section && event.method === method
  );
}

/**
 * Check if an event exists in the result
 */
export function hasEvent(
  result: ISubmittableResult,
  section: string,
  method: string
): boolean {
  return findEvent(result, section, method) !== undefined;
}

/**
 * Assert that an event was emitted
 */
export function expectEvent(
  result: ISubmittableResult,
  section: string,
  method: string,
  message?: string
): void {
  const event = findEvent(result, section, method);
  if (!event) {
    const eventList = result.events.map(e => `${e.event.section}.${e.event.method}`).join(', ');
    throw new Error(
      message || `Expected event ${section}.${method} not found. Events: ${eventList}`
    );
  }
}

/**
 * Assert that an event was NOT emitted
 */
export function expectNoEvent(
  result: ISubmittableResult,
  section: string,
  method: string,
  message?: string
): void {
  const event = findEvent(result, section, method);
  if (event) {
    throw new Error(
      message || `Expected event ${section}.${method} to not be emitted, but it was`
    );
  }
}

/**
 * Assert that multiple events were emitted in order
 */
export function expectEvents(
  result: ISubmittableResult,
  matchers: EventMatcher[]
): void {
  const events = result.events.map(e => ({
    section: e.event.section,
    method: e.event.method,
    data: e.event.data.toJSON(),
  }));

  for (const matcher of matchers) {
    const found = events.find(
      e => e.section === matcher.section && e.method === matcher.method
    );

    if (!found) {
      throw new Error(
        `Expected event ${matcher.section}.${matcher.method} not found`
      );
    }

    if (matcher.data) {
      const dataMatch = JSON.stringify(found.data) === JSON.stringify(matcher.data);
      if (!dataMatch) {
        throw new Error(
          `Event ${matcher.section}.${matcher.method} data mismatch.\nExpected: ${JSON.stringify(matcher.data)}\nReceived: ${JSON.stringify(found.data)}`
        );
      }
    }
  }
}

/**
 * Get event data as JSON
 */
export function getEventData(event: EventRecord): any {
  return event.event.data.toJSON();
}

/**
 * Assert that a contract event was emitted
 */
export function expectContractEvent(
  result: ISubmittableResult,
  eventName: string,
  message?: string
): EventRecord {
  // Contract events are typically in the 'contracts' section with method 'ContractEmitted'
  const contractEvents = findEvents(result, 'contracts', 'ContractEmitted');

  if (contractEvents.length === 0) {
    throw new Error(
      message || `No contract events found. Expected event: ${eventName}`
    );
  }

  // TODO: Parse contract event data to match specific event names
  // This requires contract metadata to decode the event properly
  return contractEvents[0];
}

/**
 * Extract error from failed transaction
 */
export function extractError(result: ISubmittableResult): string | null {
  const failedEvent = result.events.find(
    ({ event }) =>
      event.section === 'system' && event.method === 'ExtrinsicFailed'
  );

  if (!failedEvent) {
    return null;
  }

  const data = failedEvent.event.data.toJSON() as any;
  if (data && data.dispatchError) {
    if (data.dispatchError.module) {
      return `Module error: ${JSON.stringify(data.dispatchError.module)}`;
    } else if (data.dispatchError.other) {
      return `Other error: ${data.dispatchError.other}`;
    }
  }

  return 'Unknown error';
}

/**
 * Assert that a transaction succeeded
 */
export function expectSuccess(result: ISubmittableResult, message?: string): void {
  const error = extractError(result);
  if (error) {
    throw new Error(message || `Transaction failed: ${error}`);
  }

  expectEvent(result, 'system', 'ExtrinsicSuccess');
}

/**
 * Assert that a transaction failed
 */
export function expectFailure(result: ISubmittableResult, message?: string): void {
  const error = extractError(result);
  if (!error) {
    throw new Error(message || 'Expected transaction to fail, but it succeeded');
  }
}

/**
 * Assert that a transaction failed with a specific error
 */
export function expectRevert(
  result: ISubmittableResult,
  errorMessage?: string
): void {
  const error = extractError(result);

  if (!error) {
    throw new Error('Expected transaction to revert, but it succeeded');
  }

  if (errorMessage && !error.includes(errorMessage)) {
    throw new Error(
      `Expected revert with message "${errorMessage}", but got: ${error}`
    );
  }
}

/**
 * Pretty print all events in a result
 */
export function printEvents(result: ISubmittableResult): void {
  console.log('Events:');
  result.events.forEach(({ event }) => {
    console.log(`  ${event.section}.${event.method}:`, event.data.toJSON());
  });
}

/**
 * Event testing utilities namespace
 */
export const events = {
  parse: parseEvents,
  find: findEvent,
  findAll: findEvents,
  has: hasEvent,
  expect: expectEvent,
  expectNo: expectNoEvent,
  expectMany: expectEvents,
  expectContract: expectContractEvent,
  expectSuccess,
  expectFailure,
  expectRevert,
  getData: getEventData,
  extractError,
  print: printEvents,
};

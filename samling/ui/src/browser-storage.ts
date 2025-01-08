import { Err, Ok, Result } from "oxide.ts";

const KEY = "APP_STATE_V1";

export type LoadStateFailure = string;
export type SaveStateFailure = string;

export function loadState(): Result<object, LoadStateFailure> {
  let serializedState;
  try {
    serializedState = localStorage.getItem(KEY);
  } catch (e) {
    return Err(`Failed to load state from localStorage with error: ${e}`);
  }
  if (serializedState) {
    try {
      return Ok(JSON.parse(serializedState));
    } catch (e) {
      return Err(`Failed to parse state from localStorage with error: ${e}`);
    }
  } else {
    return Ok({});
  }
}

export function saveState(state: object): Result<boolean, SaveStateFailure> {
  let serializedState;
  try {
    serializedState = JSON.stringify(state);
  } catch (e) {
    return Err(`Failed to serialize state with error: ${e}`);
  }
  try {
    localStorage.setItem(KEY, serializedState);
  } catch (e) {
    return Err(`Failed to store state in localStorage with error: ${e}`);
  }
  return Ok(true);
}

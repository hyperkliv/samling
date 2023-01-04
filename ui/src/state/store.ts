import { combineReducers, configureStore } from "@reduxjs/toolkit";
import { loadState } from "../browser-storage";
import merge from "lodash.merge";
import cloneDeep from "lodash.clonedeep";
import userSlice from "./slices/user";

const rootReducer = combineReducers({
  user: userSlice.reducer,
});

const previouslyStoredState = loadState();
previouslyStoredState.mapErr((err) => {
  console.error(err);
});

const preloadedState = merge(
  { user: cloneDeep(userSlice.getInitialState()) },
  previouslyStoredState.unwrapOr({}),
);

const store = configureStore({
  reducer: rootReducer,
  preloadedState,
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export default store;

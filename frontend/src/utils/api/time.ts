import { axios } from "./axios";
import { Time, DateTimeApiPayload } from "@/models/time";

export const setTime = (t: Time | DateTimeApiPayload) => {
  try {
    axios.put("/time", t);
  } catch (err) {
    console.log(err);
  }
};

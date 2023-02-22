import { axios } from "./axios";
import { Time } from "@/models/time";

export const setTime = (t: Time) => {
  try {
    axios.put("/time", t);
  } catch (err) {
    console.log(err);
  }
};

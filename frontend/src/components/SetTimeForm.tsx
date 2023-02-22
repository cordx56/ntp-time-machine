import { useState, useEffect } from "react";

import { Time } from "@/models/time";
import { setTime } from "@/utils/api/time";

const SetTimeForm = () => {
  const d = new Date();
  const [formTime, setFormTime] = useState(
    `${d.getUTCFullYear()}-${
      d.getUTCMonth() + 1
    }-${d.getUTCDate()} ${d.getUTCHours()}:${d.getUTCMinutes()}:${d.getUTCSeconds()}`
  );
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();

        setTime({ date_time: formTime });
      }}
    >
      <div>
        date:{" "}
        <input
          type="date"
          className="input-date"
          onChange={(e) => {
            const val = e.target.value;
            setFormTime(`${val} ${formTime.split(" ")[1]}`);
          }}
        />
      </div>
      <div>
        time:{" "}
        <input
          type="time"
          className="input-time"
          onChange={(e) => {
            const val = e.target.value;
            setFormTime(`${formTime.split(" ")[0]} ${val}:00`);
          }}
        />
      </div>
      <div>
        <button type="submit" className="button">
          Set!
        </button>
      </div>
    </form>
  );
};

export default SetTimeForm;

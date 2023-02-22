import { useState, useEffect } from "react";

import { Time } from "@/models/time";
import { setTime } from "@/utils/api/time";

const SetTimeForm = () => {
  const d = new Date();
  const [formTime, setFormTime] = useState<Time>({
    year: d.getUTCFullYear(),
    month: d.getUTCMonth(),
    day: d.getUTCDay(),

    hour: d.getUTCHours(),
    minute: d.getUTCMinutes(),
    second: d.getUTCMinutes(),
  });
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();

        setTime(formTime);
      }}
    >
      <div>
        date:{" "}
        <input
          type="date"
          className="input-date"
          onChange={(e) => {
            const val = e.target.value.split("-");
            const year = parseInt(val[0]);
            const month = parseInt(val[1]);
            const day = parseInt(val[2]);
            setFormTime({ ...formTime, year, month, day });
          }}
        />
      </div>
      <div>
        time:{" "}
        <input
          type="time"
          className="input-time"
          onChange={(e) => {
            const val = e.target.value.split(":");
            const hour = parseInt(val[0]);
            const minute = parseInt(val[1]);
            const second = 0;
            setFormTime({ ...formTime, hour, minute, second });
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

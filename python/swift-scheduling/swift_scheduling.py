from datetime import datetime, timedelta, time
import calendar

DATETIME_FORMAT = "%Y-%m-%dT%H:%M:%S"
AFTERNOON = time(13, 0)
EVENING = time(17, 0)
LATE = time(20, 0)
MORNING = time(8, 0)

def _parse(dt_str: str) -> datetime:
    return datetime.strptime(dt_str, DATETIME_FORMAT)

def _format(dt: datetime) -> str:
    return dt.replace(microsecond=0).isoformat()

def _set_time(dt: datetime, t: time) -> datetime:
    return dt.replace(hour=t.hour, minute=t.minute, second=t.second, microsecond=0)

def _next_weekday_on_or_after(dt: datetime) -> datetime:
    while dt.weekday() > 4:  # Sat=5, Sun=6
        dt += timedelta(days=1)
    return dt

def _prev_weekday_on_or_before(dt: datetime) -> datetime:
    while dt.weekday() > 4:
        dt -= timedelta(days=1)
    return dt

def delivery_date(start: str, description: str) -> str:
    dt = _parse(start)

    if description == "NOW":
        return _format(dt + timedelta(hours=2))

    if description == "ASAP":
        if dt.time() < AFTERNOON:
            return _format(_set_time(dt, EVENING))
        return _format(_set_time(dt + timedelta(days=1), AFTERNOON))

    if description == "EOW":
        wd = dt.weekday()  # Mon=0
        if wd <= 2:  # Mon-Wed -> Fri 17:00
            target = dt + timedelta(days=(4 - wd))
            return _format(_set_time(target, EVENING))
        # Thu-Fri -> Sun 20:00
        target = dt + timedelta(days=(6 - wd))
        return _format(_set_time(target, LATE))

    if description.endswith("M"):
        try:
            target_month = int(description[:-1])
            if not 1 <= target_month <= 12:
                raise ValueError
        except ValueError:
            raise ValueError("Invalid month description")
        year = dt.year + (1 if dt.month >= target_month else 0)
        target = datetime(year, target_month, 1)
        target = _next_weekday_on_or_after(target)
        return _format(_set_time(target, MORNING))

    if description.startswith("Q"):
        try:
            quarter = int(description[1])
            if not 1 <= quarter <= 4:
                raise ValueError
        except (IndexError, ValueError):
            raise ValueError("Invalid quarter description")
        current_quarter = (dt.month - 1) // 3 + 1
        year = dt.year + (1 if current_quarter > quarter else 0)
        month = quarter * 3
        last_day = calendar.monthrange(year, month)[1]
        target = datetime(year, month, last_day)
        target = _prev_weekday_on_or_before(target)
        return _format(_set_time(target, MORNING))

    raise ValueError("Unsupported description")

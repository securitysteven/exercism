from datetime import datetime, timedelta, time
from typing import Optional, Union

DateLike = Union[str, datetime]

def is_workday(dt: datetime) -> bool:
    return dt.weekday() < 5

def _to_dt(start: DateLike) -> datetime:
    return start if isinstance(start, datetime) else datetime.fromisoformat(start)

def _fmt(dt: datetime) -> str:
    return dt.replace(microsecond=0).isoformat(sep="T")

def delivery_date(start: DateLike, description: str) -> Optional[str]:
    dt = _to_dt(start)

    if description == "NOW":
        return _fmt(dt + timedelta(hours=2))

    if description == "ASAP":
        if dt.time() < time(13, 0):
            return _fmt(dt.replace(hour=17, minute=0, second=0, microsecond=0))
        return _fmt((dt + timedelta(days=1)).replace(hour=17, minute=0, second=0, microsecond=0))

    if description == "EOW":
        if not is_workday(dt):
            return None
        if dt.weekday() in (0, 1, 2):  # Mon/Tue/Wed -> this week's Friday 17:00
            target = dt + timedelta(days=(4 - dt.weekday()))
            return _fmt(target.replace(hour=17, minute=0, second=0, microsecond=0))
        if dt.weekday() == 3:  # Thursday -> Sunday 20:00
            target = dt + timedelta(days=(6 - dt.weekday()))
            return _fmt(target.replace(hour=20, minute=0, second=0, microsecond=0))
        if dt.weekday() == 4:  # Friday -> same day 20:00
            return _fmt(dt.replace(hour=20, minute=0, second=0, microsecond=0))

    return None

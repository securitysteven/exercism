from datetime import datetime, timedelta, time
from typing import Optional


def is_workday(dt: datetime) -> bool:
    # Mon-Fri
    return dt.weekday() < 5

def delivery_date(start: datetime, description: str) -> Optional[datetime]:
    # Using python 3.10, this should be replaced with 'match'
    if description is "NOW":
            return start + timedelta(hours=2)
    elif description is "ASAP":
        if start.time() <  time(13, 0):
            return start.replace(hour=17, minute=0, second=0, microsecond=0)
        else:
            return start + timedelta(days=1)
    elif description is "EOW" and is_workday(start):
        if start.weekday() in (0, 1, 2):
            #  days until Friday (4)
            start = start + timedelta(days=(4 - start.weekday()))
            return start.replace(hour=17, minute=0, second=0, microsecond=0)
        else: # Thursdays, Fridays
            #  days until Sunday (4)
            start = start + timedelta(days=(7 - start.weekday()))
            return start.replace(hour=20, minute=0, second=0, microsecond=0)


from typing import List

def _validate(garden: List[str]) -> None:
    if not isinstance(garden, list):
        raise ValueError("The board is invalid with current input.")
    if len(garden) == 0:
        return
    row_len = len(garden[0])
    for row in garden:
        if not isinstance(row, str) or len(row) != row_len:
            raise ValueError("The board is invalid with current input.")
        for ch in row:
            if ch not in (" ", "*"):
                raise ValueError("The board is invalid with current input.")

def annotate(garden: List[str]) -> List[str]:
    """
    Given a garden (list of strings) where ' ' is empty and '*' is a flower,
    return a new garden where each empty square is replaced by the count of
    adjacent flowers (8-neighbourhood). Empty squares with zero adjacent
    flowers remain as ' '.

    Raises ValueError("The board is invalid with current input.") for malformed input.
    """
    _validate(garden)

    if not garden:
        return []

    rows = len(garden)
    cols = len(garden[0])
    grid = [list(row) for row in garden]

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == " ":
                count = 0
                for dr in (-1, 0, 1):
                    for dc in (-1, 0, 1):
                        if dr == 0 and dc == 0:
                            continue
                        nr, nc = r + dr, c + dc
                        if 0 <= nr < rows and 0 <= nc < cols and garden[nr][nc] == "*":
                            count += 1
                if count > 0:
                    grid[r][c] = str(count)
    return ["".join(row) for row in grid]

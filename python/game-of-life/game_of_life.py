# Any live cell with two or three live neighbors lives on.
# Any dead cell with exactly three live neighbors becomes a live cell.
# All other cells die or stay dead.

def tick(matrix):
    if not matrix:
        return []

    rows = len(matrix)
    cols = len(matrix[0]) if rows else 0
    new_matrix = [[0] * cols for _ in range(rows)]

    for r in range(rows):
        for c in range(cols):
            live_neighbors = 0

            for dr in (-1, 0, 1):
                for dc in (-1, 0, 1):
                    if dr == 0 and dc == 0:
                        continue
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < rows and 0 <= nc < cols:
                        live_neighbors += matrix[nr][nc]

            # Apply rules
            if matrix[r][c] == 1:
                new_matrix[r][c] = 1 if live_neighbors in (2, 3) else 0
            else:
                new_matrix[r][c] = 1 if live_neighbors == 3 else 0

    return new_matrix
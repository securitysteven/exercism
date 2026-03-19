# Rules:
#
# Numbers ending in 1 (unless ending in 11) → "st"
# Numbers ending in 2 (unless ending in 12) → "nd"
# Numbers ending in 3 (unless ending in 13) → "rd"
# All other numbers → "th"

def line_up(name, number):
    if 11 <= number % 100 <= 13:
        suffix = "th"
    else:
        suffix = ("th", "st", "nd", "rd", "th", "th", "th", "th", "th", "th")[number % 10]

    return f"{name}, you are the {number}{suffix} customer we serve today. Thank you!"

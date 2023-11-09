import random

num_points = 500
data = f"{num_points}\n"

for _ in range(num_points):
    x = round(random.uniform(1, 100), 4)
    y = round(random.uniform(1, 100), 4)
    data += f"{x} {y}\n"

print(data)

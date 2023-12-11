import math
import random

points = [
    (95.0129, 61.5432),
    (23.1139, 79.1937),
    (60.6843, 92.1813),
    (48.5982, 73.8207),
    (89.1299, 17.6266),
    (76.2097, 40.5706),
    (45.6468, 93.5470),
    (1.8504, 91.6904),
    (82.1407, 41.0270),
    (44.4703, 89.3650)
]


def distance(p1, p2):
    return math.sqrt((p1[0] - p2[0])**2 + (p1[1] - p2[1])**2)


def total_distance(path, points):
    dist = sum(distance(points[path[i]], points[path[i + 1]]) for i in range(len(path) - 1))
    dist += distance(points[path[-1]], points[path[0]])  # Return to start
    return dist


def two_opt_swap(route, i, k):
    return route[:i] + route[i:k+1][::-1] + route[k+1:]


def move_firefly(firefly_i, firefly_j, points, beta0, gamma):
    new_firefly_i = firefly_i[:]
    dist = total_distance(firefly_i, points) - total_distance(firefly_j, points)
    beta = beta0 * math.exp(-gamma * dist)

    for _ in range(int(beta * len(firefly_i))):
        i, k = sorted(random.sample(range(len(firefly_i)), 2))

        new_firefly_i = two_opt_swap(new_firefly_i, i, k)
        if total_distance(new_firefly_i, points) < total_distance(firefly_i, points):
            firefly_i = new_firefly_i

    return firefly_i


num_fireflies = 50
fireflies = [random.sample(range(10), 10) for _ in range(num_fireflies)]
beta0 = 5.0
gamma = 0.1

max_generations = 10
for _ in range(max_generations):
    intensities = [-total_distance(f, points) for f in fireflies]
    for i in range(num_fireflies):
        for j in range(num_fireflies):
            if intensities[i] < intensities[j]:
                fireflies[i] = move_firefly(fireflies[i], fireflies[j], points, beta0, gamma)
                intensities[i] = -total_distance(fireflies[i], points)

best_firefly_index = intensities.index(max(intensities))
best_path = fireflies[best_firefly_index]
best_distance = total_distance(best_path, points)

print("Best path found:", best_path)
print("Total distance:", best_distance)

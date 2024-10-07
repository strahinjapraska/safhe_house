import safhe_house
import matplotlib.pyplot as plt

samples = 200000

samples = [safhe_house.sample_discrete_gaussian(8.0, 2048) for _ in range(samples)]

min_sample = int(min(samples))
max_sample = int(max(samples))

bins = range(min_sample, max_sample + 1)

plt.hist(samples, bins=bins, density=True, alpha=0.7, color="blue", align='left')

plt.xticks([])
plt.yticks([])

plt.show()
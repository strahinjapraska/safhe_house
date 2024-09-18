import safhe_house 
import matplotlib.pyplot as plt 

samples = 100000

samples = [safhe_house.sample_discrete_gaussian(2.3, 1000) for _ in range(samples)]
plt.hist(samples, bins = 30, density=True, alpha = 0.7, color = "blue") 

plt.show()
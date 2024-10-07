import matplotlib.pyplot as plt 
import pandas as pd
import os


project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), '../')) 
csv_file_path = os.path.join(project_root, 'bin/benches/timings/poly_mul_benchmark_time.csv') 


df = pd.read_csv(csv_file_path, header=None, names=["Karatsuba", "FFT", "Schoolbook", "NTT", "Degree"])

df_filtered_schoolbook = df[df["Schoolbook"] != 0]

plt.figure(figsize=(10, 6))
plt.plot(df["Degree"], df["Karatsuba"], label="Karatsuba", marker='o')
plt.plot(df["Degree"], df["FFT"], label="FFT", marker='o')
plt.plot(df_filtered_schoolbook["Degree"], df_filtered_schoolbook["Schoolbook"], label="Schoolbook", marker='o')
plt.plot(df["Degree"], df["NTT"], label="NTT", marker='o')

plt.xlabel("Degree (n)")
plt.ylabel("Time Taken")
plt.legend(title="Algorithms")
plt.grid(True)

plt.show()
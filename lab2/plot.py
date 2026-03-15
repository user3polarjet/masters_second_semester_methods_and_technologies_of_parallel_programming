import pandas as pd
import matplotlib.pyplot as plt
import os

configs = {
    "html.csv": {
        "names": ["method", "time", "tags"], 
        "title": "Підрахунок частоти HTML тегів", 
        "color": ['#4C72B0', '#DD8452', '#55A868', '#C44E52']
    },
    "numbers.csv": {
        "names": ["method", "time", "min", "max", "median", "mean"], 
        "title": "Статистика масиву", 
        "color": ['#4C72B0', '#DD8452', '#55A868', '#C44E52']
    },
    "matrix.csv": {
        "names": ["method", "time", "sum"], 
        "title": "Множення матриць", 
        "color": ['#4C72B0', '#DD8452', '#55A868', '#C44E52']
    },
    "image_processing.csv": {
        "names": ["method", "time"], 
        "title": "Обробка зображень (Pipeline vs Worker Pool)", 
        "color": ['#8172B3', '#937860']
    }
}

def generate_plots():
    for filename, config in configs.items():
        df = pd.read_csv(filename, header=None, names=config["names"])
        plt.figure(figsize=(10, 6))
        colors = config["color"][:len(df)]
        bars = plt.bar(df["method"], df["time"], color=colors, edgecolor='black', linewidth=0.5)
        plt.grid(axis='y', linestyle='--', alpha=0.7)
        plt.gca().set_axisbelow(True)
        plt.title(config["title"], fontsize=15, pad=20, fontweight='bold')
        plt.ylabel("Час виконання (секунди)", fontsize=12)
        plt.xlabel("Патерн / Алгоритм", fontsize=12)
        plt.xticks(fontsize=11)
        plt.yticks(fontsize=11)
        for bar in bars:
            yval = bar.get_height()
            plt.text(
                bar.get_x() + bar.get_width()/2, 
                yval + (max(df["time"]) * 0.02),
                f"{yval:.4f}s", 
                ha='center', 
                va='bottom', 
                fontsize=11, 
                fontweight='bold',
                color='#333333'
            )
            
        plt.tight_layout()
        out_name = filename.replace('.csv', '.svg')
        plt.savefig(out_name, format='svg')
        plt.close()

if __name__ == "__main__":
    generate_plots()

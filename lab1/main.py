import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

def main():
    words_single = pd.read_csv('count_words_single.csv')
    words_multi = pd.read_csv('count_words_multi.csv')
    words_single()
    pass

if __name__ == '__main__':
    main()
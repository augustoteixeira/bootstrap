#!/usr/bin/python3
from math import exp
import math
import numpy as np
from sklearn.linear_model import LinearRegression
from scipy.optimize import curve_fit
import matplotlib.pyplot as plt
import csv

with open('./report.csv', newline='') as csvfile:

    reader = csv.reader(csvfile, delimiter=',', quotechar='|')
    pls = []
    p = []
    for row in reader:
        pls.append(float(row[0]))
        p.append(float(row[1]))

pp = []
for m in range(2, 12):
    pp.append(2**(-m))

pls = np.array(pls)
pp = np.array(pp)

ls = np.divide(pls, pp)



################## Estimating alpha and a ########################

lp = np.log(pp[3:-1]) # trim first points
ll = np.log(ls[3:-1])
reg = LinearRegression().fit(lp.reshape(-1,1), ll)
coef = reg.coef_[0]
inter = reg.intercept_

plt.subplot(2, 2, 1)
plt.title("Estimate alpha and a: log(log(s)) ~ log(a) + alpha log(p)")
plt.xlabel("log(p)")
plt.ylabel("log(log(s))")
plt.scatter(lp, ll)
plt.plot([lp[0], lp[-1]], [inter + coef * lp[0], inter + coef * lp[-1]])
plt.legend(["alpha ~ {:.6f}, a ~ {:.6f}".format(coef, exp(inter))])

################## Estimating a and beta, knowing alpha = 1 ########################

pinv = np.divide(1, pp[4:-1])
plst = pls[4:-1]
def polydecay(x, a, b, beta_plus_one):
    return a + b / pow(x, beta_plus_one)

popt, pcov = curve_fit(polydecay, pinv, plst, p0=[3.2, -10, 0.7], maxfev=5000)

plt.subplot(2, 2, 2)
plt.title("Estimate a, beta and b: p*log(s) ~ a + b p^(beta + 1)")
plt.xlabel("1 / p")
plt.ylabel("p * log(s)")
plt.scatter(pinv, plst)
plt.plot(pinv, polydecay(pinv, *popt), 'g--',
         label='fit: a = %5.3f, b = %5.3f, beta = %5.3f' % (popt[0], popt[1], popt[2] - 1))
plt.legend()


################## Estimating beta and b, knowing alpha and a ########################

a = math.pi**2/3
a_minus_log_pls = np.log(a - pls[3:-1])

reg = LinearRegression().fit(lp.reshape(-1,1), a_minus_log_pls)
coef = reg.coef_[0]
inter = reg.intercept_

plt.subplot(2, 2, 3)
plt.title("Estimate beta and b: log(p*log(s) - a) ~ log(b) + beta * log(p)")
plt.xlabel("log(p)")
plt.ylabel("log(p * log(s) - a)")
plt.scatter(lp, a_minus_log_pls)
plt.plot([lp[0], lp[-1]], [inter + coef * lp[0], inter + coef * lp[-1]],
         label='fit: beta = %5.3f, b = %5.3f' % (coef - 1, -exp(inter)))
plt.legend()


################## Estimating b and gamma, knowing alpha, a and beta ########################

am_over_sqrt = np.divide(a_minus_log_pls, np.sqrt(pp[3:-1]))
pinv_2 = np.divide(1, pp[3:-1])

def polydecay(x, a, b, beta_plus_one):
    return a + b / pow(x, beta_plus_one)

#popt, pcov = curve_fit(polydecay, pinv_2, am_over_sqrt, p0=[3.2, -10, 0.5])

plt.subplot(2, 2, 4)
plt.title("Estimate b, gamma and c: (a - p*log(s))/sqrt(p) ~ - b - c (1/p)^(gamma + 1.5)")
plt.xlabel("1 / p")
plt.ylabel("(a - p * log(s))/sqrt(p)")
plt.scatter(pinv_2, am_over_sqrt)
#plt.plot(pinv_2, polydecay(pinv_2, *popt), 'g--',
#         label='fit: a = %5.3f, b = %5.3f, beta = %5.3f' % (popt[0], popt[1], popt[2] - 1))
plt.legend()





plt.show()

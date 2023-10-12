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
    m_array = []
    pls = []
    for row in reader:
        m_array.append(int(row[0]))
        pls.append(float(row[1]))


pp = []
for m in m_array:
    pp.append(2**(-m))

pls = np.array(pls)
pp = np.array(pp)

ls = np.divide(pls, pp)

################## A - Estimate alpha and a ########################

truncateA = 7

lp = np.log(pp)
ll = np.log(ls)
reg = LinearRegression().fit(lp[truncateA:].reshape(-1,1), ll[truncateA:])
coef = reg.coef_[0]
inter = reg.intercept_

plt.subplot(2, 3, 1)
plt.title("Estimate alpha and a: log(log(s)) ~ log(a) + alpha log(p)")
plt.xlabel("-log(p)")
plt.ylabel("-log(log(s))")
plt.scatter(-lp[0:truncateA], -ll[0:truncateA], color="red")
plt.scatter(-lp[truncateA:], -ll[truncateA:], color="blue")
plt.plot([-lp[truncateA], -lp[-1]], [-inter - coef * lp[truncateA], -inter - coef * lp[-1]])
plt.legend(["alpha ~ {:.6f}, a ~ {:.6f}".format(coef, -exp(inter))])

################## B - Estimate a and beta | alpha ########################

truncateB = 7

pinvB = np.divide(1, pp)
plstB = pls

def polydecay(x, a, b, beta_plus_one):
    return a + b / pow(x, beta_plus_one)
popt, pcov = curve_fit(polydecay, pinvB[truncateB:], plstB[truncateB:],
                       p0=[3.2, -10, 0.7], maxfev=5000)

plt.subplot(2, 3, 2)
plt.title("Estimate a, beta and b: p*log(s) ~ a + b p^(beta + 1)")
plt.xlabel("1 / p")
plt.ylabel("p * log(s)")
plt.scatter(pinvB[0:truncateB], plstB[0:truncateB], color="red")
plt.scatter(pinvB[truncateB:], plstB[truncateB:], color="blue")
plt.plot(pinvB[truncateB:], polydecay(pinvB[truncateB:], *popt), 'g--',
         label='fit: a = %5.3f, b = %5.3f, beta = %5.3f' % (popt[0], popt[1], popt[2] - 1))
plt.legend()

################## C - Estimate beta and b | alpha, a ########################

truncateC = 9

a = math.pi**2/3
log_a_minus_pls = np.log(a - pls)

reg = LinearRegression().fit(lp[truncateC:].reshape(-1,1), log_a_minus_pls[truncateC:])
coef = reg.coef_[0]
inter = reg.intercept_

plt.subplot(2, 3, 3)
plt.title("Estimate beta and b: log(p*log(s) - a) ~ log(b) + beta * log(p)")
plt.xlabel("-log(p)")
plt.ylabel("-log(p * log(s) - a)")
plt.scatter(-lp[0:truncateC], log_a_minus_pls[0:truncateC], color="red")
plt.scatter(-lp[truncateC:], log_a_minus_pls[truncateC:], color="blue")
plt.plot([-lp[truncateC], -lp[-1]], [inter + coef * lp[truncateC], inter + coef * lp[-1]],
         label='fit: beta = %5.3f, b = %5.3f' % (coef - 1, -exp(inter)))
plt.legend()

################## D - Estimate b and gamma | alpha, a and beta ########################

truncateD = 7

pls_minus_a = pls - a
am_over_sqrt = np.divide(pls_minus_a, np.sqrt(pp))
pinv_2 = np.divide(1, pp)

def polydecay(x, b, c, gamma_plus_half):
    return b + c / pow(x, gamma_plus_half)
popt, pcov = curve_fit(polydecay, pinv_2[truncateD:], am_over_sqrt[truncateD:],
                       p0=[-8, -3, 0.2], maxfev=5000)

plt.subplot(2, 3, 4)
plt.title("Estimate b, gamma and c: (a - p*log(s))/sqrt(p) ~ - b - c (1/p)^(gamma + 0.5)")
plt.xlabel("1 / p")
plt.ylabel("(a - p * log(s))/sqrt(p)")
plt.scatter(pinv_2[0:truncateD], am_over_sqrt[0:truncateD], color="red")
plt.scatter(pinv_2[truncateD:], am_over_sqrt[truncateD:], color="blue")
plt.plot(pinv_2[truncateD:], polydecay(pinv_2[truncateD:], *popt), 'g--',
         label='fit: b = %5.3f, c = %5.3f, gamma = %5.3f' % (popt[0], popt[1], popt[2] - 0.5))
plt.legend()

################## E - Estimate b and c | alpha, a, beta, gamme  ########################

truncateE = 9

a = math.pi**2/3
psixth = np.power(pp, 1/6)
psixth_inv = np.power(np.divide(1, pp), 1/6)
reg = LinearRegression().fit(psixth_inv[truncateE:].reshape(-1,1), am_over_sqrt[truncateE:])
coef = reg.coef_[0]
inter = reg.intercept_

plt.subplot(2, 3, 6)
plt.title("Estimate b and c: (a - p*log(s))/sqrt(p) ~ - b - c (1/p)^(1/6)")
plt.xlabel("1 / p^(1/6)")
plt.ylabel("(a - p * log(s))/sqrt(p)")
plt.scatter(psixth_inv[0:truncateE], am_over_sqrt[0:truncateE], color="red")
plt.scatter(psixth_inv[truncateE:], am_over_sqrt[truncateE:], color="blue")
plt.plot([psixth_inv[truncateE], psixth_inv[-1]],
         [inter + coef * psixth_inv[truncateE], inter + coef * psixth_inv[-1]],
         label='fit: c = %5.3f, b = %5.3f' % (coef, inter))
plt.legend()



plt.show()

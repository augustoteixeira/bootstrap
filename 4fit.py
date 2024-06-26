import numpy as np
m=[2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]
P=[0.9115734772261084,1.1856678944832488,1.5491163790750622,1.9092077283425182,2.225657720143466,2.486014696215505,2.691608897953176,2.849567869062738,2.9685893044529106,3.056992807612127,3.1219472776204498,3.1692778108239446,3.2035453214237313,3.2282313318498783,3.245945506007917,3.2586179365262704]
from scipy.optimize import curve_fit
def func(mm, a, b, c, d):
    return a * (2**(b*mm))-c*(2**((b-d)*mm))
xdata = np.array(m[-4:])
ydata = np.array([P[i]*2**(m[i]-1) for i in range(len(m)-4,len(m))])
popt, pcov = curve_fit(func, xdata, ydata, bounds=([0.,0.,-np.inf,0.01],np.inf))
print(tuple(popt))

//! H2O coefficients at 25°C.
//!
//! # Resources
//! Data taken from [here](https://refractiveindex.info/?shelf=3d&book=liquids&page=water) on 2021-02-21.

use crate::Float;

pub static INDEX: [Float; 169] = {
    [
        200.0, 225.0, 250.0, 275.0, 300.0, 325.0, 350.0, 375.0, 400.0, 425.0, 450.0, 475.0, 500.0,
        525.0, 550.0, 575.0, 600.0, 625.0, 650.0, 675.0, 700.0, 725.0, 750.0, 775.0, 800.0, 825.0,
        850.0, 875.0, 900.0, 925.0, 950.0, 975.0, 1000.0, 1200.0, 1400.0, 1600.0, 1800.0, 2000.0,
        2200.0, 2400.0, 2600.0, 2650.0, 2700.0, 2750.0, 2800.0, 2850.0, 2900.0, 2950.0, 3000.0,
        3050.0, 3100.0, 3150.0, 3200.0, 3250.0, 3300.0, 3350.0, 3400.0, 3450.0, 3500.0, 3600.0,
        3700.0, 3800.0, 3900.0, 4000.0, 4100.0, 4200.0, 4300.0, 4400.0, 4500.0, 4600.0, 4700.0,
        4800.0, 4900.0, 5000.0, 5100.0, 5200.0, 5300.0, 5400.0, 5500.0, 5600.0, 5700.0, 5800.0,
        5900.0, 6000.0, 6100.0, 6200.0, 6300.0, 6400.0, 6500.0, 6600.0, 6700.0, 6800.0, 6900.0,
        7000.0, 7100.0, 7200.0, 7300.0, 7400.0, 7500.0, 7600.0, 7700.0, 7800.0, 7900.0, 8000.0,
        8200.0, 8400.0, 8600.0, 8800.0, 9000.0, 9200.0, 9400.0, 9600.0, 9800.0, 10000.0, 10500.0,
        11000.0, 11500.0, 12000.0, 12500.0, 13000.0, 13500.0, 14000.0, 14500.0, 15000.0, 15500.0,
        16000.0, 16500.0, 17000.0, 17500.0, 18000.0, 18500.0, 19000.0, 19500.0, 20000.0, 21000.0,
        22000.0, 23000.0, 24000.0, 25000.0, 26000.0, 27000.0, 28000.0, 29000.0, 30000.0, 32000.0,
        34000.0, 36000.0, 38000.0, 40000.0, 42000.0, 44000.0, 46000.0, 48000.0, 50000.0, 60000.0,
        70000.0, 80000.0, 90000.0, 100000.0, 110000.0, 120000.0, 130000.0, 140000.0, 150000.0,
        160000.0, 170000.0, 180000.0, 190000.0, 200000.0,
    ]
};
pub static N: [Float; 169] = {
    [
        1.396, 1.373, 1.362, 1.354, 1.349, 1.346, 1.343, 1.341, 1.339, 1.338, 1.337, 1.336, 1.335,
        1.334, 1.333, 1.333, 1.332, 1.332, 1.331, 1.331, 1.331, 1.33, 1.33, 1.33, 1.329, 1.329,
        1.329, 1.328, 1.328, 1.328, 1.327, 1.327, 1.327, 1.324, 1.321, 1.317, 1.312, 1.306, 1.296,
        1.279, 1.242, 1.219, 1.188, 1.157, 1.142, 1.149, 1.201, 1.292, 1.371, 1.426, 1.467, 1.483,
        1.478, 1.467, 1.45, 1.432, 1.42, 1.41, 1.4, 1.385, 1.374, 1.364, 1.357, 1.351, 1.346,
        1.342, 1.338, 1.334, 1.332, 1.33, 1.33, 1.33, 1.328, 1.325, 1.322, 1.317, 1.312, 1.305,
        1.298, 1.289, 1.277, 1.262, 1.248, 1.265, 1.319, 1.363, 1.357, 1.347, 1.339, 1.334, 1.329,
        1.324, 1.321, 1.317, 1.314, 1.312, 1.309, 1.307, 1.304, 1.302, 1.299, 1.297, 1.294, 1.291,
        1.286, 1.281, 1.275, 1.269, 1.262, 1.255, 1.247, 1.239, 1.229, 1.218, 1.185, 1.153, 1.126,
        1.111, 1.123, 1.146, 1.177, 1.21, 1.241, 1.27, 1.297, 1.325, 1.351, 1.376, 1.401, 1.423,
        1.443, 1.461, 1.476, 1.48, 1.487, 1.5, 1.511, 1.521, 1.531, 1.539, 1.545, 1.549, 1.551,
        1.551, 1.546, 1.536, 1.527, 1.522, 1.519, 1.522, 1.53, 1.541, 1.555, 1.587, 1.703, 1.821,
        1.886, 1.924, 1.957, 1.966, 2.004, 2.036, 2.056, 2.069, 2.081, 2.094, 2.107, 2.119, 2.13,
    ]
};
pub static K: [Float; 169] = {
    [
        1.10E-07, 4.90E-08, 3.35E-08, 2.35E-08, 1.60E-08, 1.08E-08, 6.50E-09, 3.50E-09, 1.86E-09,
        1.30E-09, 1.02E-09, 9.35E-10, 1.00E-09, 1.32E-09, 1.96E-09, 3.60E-09, 1.09E-08, 1.39E-08,
        1.64E-08, 2.23E-08, 3.35E-08, 9.15E-08, 1.56E-07, 1.48E-07, 1.25E-07, 1.82E-07, 2.93E-07,
        3.91E-07, 4.86E-07, 0.00000106, 0.00000293, 0.00000348, 0.00000289, 0.00000989, 0.000138,
        0.0000855, 0.000115, 0.0011, 0.000289, 0.000956, 0.00317, 0.0067, 0.019, 0.059, 0.115,
        0.185, 0.268, 0.298, 0.272, 0.24, 0.192, 0.135, 0.0924, 0.061, 0.0368, 0.0261, 0.0195,
        0.0132, 0.0094, 0.00515, 0.0036, 0.0034, 0.0038, 0.0046, 0.00562, 0.00688, 0.00845, 0.0103,
        0.0134, 0.0147, 0.0157, 0.015, 0.0137, 0.0124, 0.0111, 0.0101, 0.0098, 0.0103, 0.0116,
        0.0142, 0.0203, 0.033, 0.0622, 0.107, 0.131, 0.088, 0.057, 0.0449, 0.0392, 0.0356, 0.0337,
        0.0327, 0.0322, 0.032, 0.032, 0.0321, 0.0322, 0.0324, 0.0326, 0.0328, 0.0331, 0.0335,
        0.0339, 0.0343, 0.0351, 0.0361, 0.0372, 0.0385, 0.0399, 0.0415, 0.0433, 0.0454, 0.0479,
        0.0508, 0.0662, 0.0968, 0.142, 0.199, 0.259, 0.305, 0.343, 0.37, 0.388, 0.402, 0.414,
        0.422, 0.428, 0.429, 0.429, 0.426, 0.421, 0.414, 0.404, 0.393, 0.382, 0.373, 0.367, 0.361,
        0.356, 0.35, 0.344, 0.338, 0.333, 0.328, 0.324, 0.329, 0.343, 0.361, 0.385, 0.409, 0.436,
        0.462, 0.488, 0.514, 0.587, 0.576, 0.547, 0.536, 0.532, 0.531, 0.526, 0.514, 0.5, 0.495,
        0.496, 0.497, 0.499, 0.501, 0.504,
    ]
};

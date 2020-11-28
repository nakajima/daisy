#![allow(clippy::unreadable_literal)]
#![allow(clippy::excessive_precision)]

pub const LENGTH: usize = 512;
type Wavetable = [f32; LENGTH];

// 512 samples @ 48 KHz = 93.75 Hz
pub static SAW: Wavetable = [
    0.0000000000, -1.0000000000, -0.7617416840, -0.8973038553,
    -0.7952467679, -0.8684955392, -0.8026719089, -0.8521037367,
    -0.8031754238, -0.8399762757, -0.8008617851, -0.8298087747,
    -0.7971288642, -0.8207023893, -0.7925816186, -0.8122346872,
    -0.7875241277, -0.8041811211, -0.7821258594, -0.7964113640,
    -0.7764886983, -0.7888446070, -0.7706775647, -0.7814280933,
    -0.7647357747, -0.7741259254, -0.7586933308, -0.7669128331,
    -0.7525716684, -0.7597705155, -0.7463865062, -0.7526853961,
    -0.7401496309, -0.7456471933, -0.7338700518, -0.7386479794,
    -0.7275547720, -0.7316815455, -0.7212093155, -0.7247429607,
    -0.7148380970, -0.7178282608, -0.7084446861, -0.7109342235,
    -0.7020319987, -0.7040582037, -0.6956024399, -0.6971980101,
    -0.6891580099, -0.6903518134, -0.6827003850, -0.6835180746,
    -0.6762309805, -0.6766954903, -0.6697509985, -0.6698829500,
    -0.6632614664, -0.6630795020, -0.6567632669, -0.6562843264,
    -0.6502571625, -0.6494967135, -0.6437438146, -0.6427160462,
    -0.6372237996, -0.6359417854, -0.6306976220, -0.6291734587,
    -0.6241657246, -0.6224106505, -0.6176284979, -0.6156529936,
    -0.6110862872, -0.6089001631, -0.6045393989, -0.6021518699,
    -0.5979881056, -0.5954078567, -0.5914326506, -0.5886678934,
    -0.5848732516, -0.5819317737, -0.5783101040, -0.5751993126,
    -0.5717433834, -0.5684703431, -0.5651732484, -0.5617447146,
    -0.5585998423, -0.5550222907, -0.5520232948, -0.5483029479,
    -0.5454437239, -0.5415865734, -0.5388612371, -0.5348730648,
    -0.5322759323, -0.5281623284, -0.5256878993, -0.5214542781,
    -0.5190972203, -0.5147488352, -0.5125039710, -0.5080459271,
    -0.5059082212, -0.5013454866, -0.4993100353, -0.4946474517,
    -0.4927094732, -0.4879517647, -0.4861065904, -0.4812583721,
    -0.4795014387, -0.4745672238, -0.4728940666, -0.4678782727,
    -0.4662845196, -0.4611914750, -0.4596728404, -0.4545067889,
    -0.4530590693, -0.4478241753, -0.4464432448, -0.4411435967,
    -0.4398254031, -0.4344650178, -0.4332055789, -0.4277884046,
    -0.4265838055, -0.4211137245, -0.4199601147, -0.4144409463,
    -0.4133345373, -0.4077700397, -0.4067071030, -0.4011009754,
    -0.4000778408, -0.3944337250, -0.3934467786, -0.3877682606,
    -0.3868139440, -0.3811045552, -0.3801793639, -0.3744425821,
    -0.3735430647, -0.3677823149, -0.3669050725, -0.3611237279,
    -0.3602654130, -0.3544667953, -0.3536241118, -0.3478114917,
    -0.3469811942, -0.3411577919, -0.3403366855, -0.3345056707,
    -0.3336906108, -0.3278551028, -0.3270429953, -0.3212060633,
    -0.3203938641, -0.3145585268, -0.3137432424, -0.3079124681,
    -0.3070911555, -0.3012678619, -0.3004376290, -0.2946246826,
    -0.2937826883, -0.2879829047, -0.2871263592, -0.2813425021,
    -0.2804686677, -0.2747034489, -0.2738096399, -0.2680657188,
    -0.2671493024, -0.2614292852, -0.2604876816, -0.2547941214,
    -0.2538248046, -0.2481602003, -0.2471606985, -0.2415274945,
    -0.2404953909, -0.2348959764, -0.2338289095, -0.2282656181,
    -0.2271612824, -0.2216363913, -0.2204925379, -0.2150082676,
    -0.2138227048, -0.2083812180, -0.2071518121, -0.2017552135,
    -0.2004798890, -0.1951302246, -0.1938069652, -0.1885062214,
    -0.1871330707, -0.1818831740, -0.1804582355, -0.1752610518,
    -0.1737824904, -0.1686398243, -0.1671058661, -0.1620194605,
    -0.1604283938, -0.1553999290, -0.1537501048, -0.1487811982,
    -0.1470710309, -0.1421632365, -0.1403912042, -0.1355460115,
    -0.1337106567, -0.1289294910, -0.1270294211, -0.1223136423,
    -0.1203475302, -0.1156984325, -0.1136650168, -0.1090838285,
    -0.1069819144, -0.1024697969, -0.1002982562, -0.0958563042,
    -0.0936140761, -0.0892433165, -0.0869294078, -0.0826308000,
    -0.0802442854, -0.0760187204, -0.0735587433, -0.0694070435,
    -0.0668728157, -0.0627957348, -0.0601865372, -0.0561847596,
    -0.0534999425, -0.0495740833, -0.0468130665, -0.0429636708,
    -0.0401259440, -0.0363534873, -0.0334386101, -0.0297434977,
    -0.0267510998, -0.0231336668, -0.0200634484, -0.0165239594,
    -0.0133756911, -0.0099143403, -0.0066878632, -0.0033047742,
    -0.0000000000, 0.0033047742, 0.0066878632, 0.0099143403,
    0.0133756911, 0.0165239594, 0.0200634484, 0.0231336668,
    0.0267510998, 0.0297434977, 0.0334386101, 0.0363534873,
    0.0401259440, 0.0429636708, 0.0468130665, 0.0495740833,
    0.0534999425, 0.0561847596, 0.0601865372, 0.0627957348,
    0.0668728157, 0.0694070435, 0.0735587433, 0.0760187204,
    0.0802442854, 0.0826308000, 0.0869294078, 0.0892433165,
    0.0936140761, 0.0958563042, 0.1002982562, 0.1024697969,
    0.1069819144, 0.1090838285, 0.1136650168, 0.1156984325,
    0.1203475302, 0.1223136423, 0.1270294211, 0.1289294910,
    0.1337106567, 0.1355460115, 0.1403912042, 0.1421632365,
    0.1470710309, 0.1487811982, 0.1537501048, 0.1553999290,
    0.1604283938, 0.1620194605, 0.1671058661, 0.1686398243,
    0.1737824904, 0.1752610518, 0.1804582355, 0.1818831740,
    0.1871330707, 0.1885062214, 0.1938069652, 0.1951302246,
    0.2004798890, 0.2017552135, 0.2071518121, 0.2083812180,
    0.2138227048, 0.2150082676, 0.2204925379, 0.2216363913,
    0.2271612824, 0.2282656181, 0.2338289095, 0.2348959764,
    0.2404953909, 0.2415274945, 0.2471606985, 0.2481602003,
    0.2538248046, 0.2547941214, 0.2604876816, 0.2614292852,
    0.2671493024, 0.2680657188, 0.2738096399, 0.2747034489,
    0.2804686677, 0.2813425021, 0.2871263592, 0.2879829047,
    0.2937826883, 0.2946246826, 0.3004376290, 0.3012678619,
    0.3070911555, 0.3079124681, 0.3137432424, 0.3145585268,
    0.3203938641, 0.3212060633, 0.3270429953, 0.3278551028,
    0.3336906108, 0.3345056707, 0.3403366855, 0.3411577919,
    0.3469811942, 0.3478114917, 0.3536241118, 0.3544667953,
    0.3602654130, 0.3611237279, 0.3669050725, 0.3677823149,
    0.3735430647, 0.3744425821, 0.3801793639, 0.3811045552,
    0.3868139440, 0.3877682606, 0.3934467786, 0.3944337250,
    0.4000778408, 0.4011009754, 0.4067071030, 0.4077700397,
    0.4133345373, 0.4144409463, 0.4199601147, 0.4211137245,
    0.4265838055, 0.4277884046, 0.4332055789, 0.4344650178,
    0.4398254031, 0.4411435967, 0.4464432448, 0.4478241753,
    0.4530590693, 0.4545067889, 0.4596728404, 0.4611914750,
    0.4662845196, 0.4678782727, 0.4728940666, 0.4745672238,
    0.4795014387, 0.4812583721, 0.4861065904, 0.4879517647,
    0.4927094732, 0.4946474517, 0.4993100353, 0.5013454866,
    0.5059082212, 0.5080459271, 0.5125039710, 0.5147488352,
    0.5190972203, 0.5214542781, 0.5256878993, 0.5281623284,
    0.5322759323, 0.5348730648, 0.5388612371, 0.5415865734,
    0.5454437239, 0.5483029479, 0.5520232948, 0.5550222907,
    0.5585998423, 0.5617447146, 0.5651732484, 0.5684703431,
    0.5717433834, 0.5751993126, 0.5783101040, 0.5819317737,
    0.5848732516, 0.5886678934, 0.5914326506, 0.5954078567,
    0.5979881056, 0.6021518699, 0.6045393989, 0.6089001631,
    0.6110862872, 0.6156529936, 0.6176284979, 0.6224106505,
    0.6241657246, 0.6291734587, 0.6306976220, 0.6359417854,
    0.6372237996, 0.6427160462, 0.6437438146, 0.6494967135,
    0.6502571625, 0.6562843264, 0.6567632669, 0.6630795020,
    0.6632614664, 0.6698829500, 0.6697509985, 0.6766954903,
    0.6762309805, 0.6835180746, 0.6827003850, 0.6903518134,
    0.6891580099, 0.6971980101, 0.6956024399, 0.7040582037,
    0.7020319987, 0.7109342235, 0.7084446861, 0.7178282608,
    0.7148380970, 0.7247429607, 0.7212093155, 0.7316815455,
    0.7275547720, 0.7386479794, 0.7338700518, 0.7456471933,
    0.7401496309, 0.7526853961, 0.7463865062, 0.7597705155,
    0.7525716684, 0.7669128331, 0.7586933308, 0.7741259254,
    0.7647357747, 0.7814280933, 0.7706775647, 0.7888446070,
    0.7764886983, 0.7964113640, 0.7821258594, 0.8041811211,
    0.7875241277, 0.8122346872, 0.7925816186, 0.8207023893,
    0.7971288642, 0.8298087747, 0.8008617851, 0.8399762757,
    0.8031754238, 0.8521037367, 0.8026719089, 0.8684955392,
    0.7952467679, 0.8973038553, 0.7617416840, 1.0000000000,
];


// 512 samples @ 48 KHz = 93.75 Hz
pub static SIN: Wavetable = [
    0., 0.012271538, 0.024541229, 0.036807224, 0.049067676,
    0.061320737, 0.07356457, 0.08579731, 0.09801714, 0.110222206,
    0.12241068, 0.1345807, 0.14673047, 0.15885815, 0.17096189,
    0.18303989, 0.19509032, 0.20711137, 0.21910124, 0.2310581,
    0.24298018, 0.25486565, 0.26671275, 0.2785197, 0.29028466,
    0.30200595, 0.31368175, 0.3253103, 0.33688986, 0.34841868,
    0.35989505, 0.3713172, 0.38268343, 0.39399204, 0.4052413,
    0.41642955, 0.42755508, 0.43861625, 0.44961134, 0.46053872,
    0.47139674,0.48218378, 0.4928982, 0.50353837, 0.51410276,
    0.52458966, 0.53499764, 0.545325, 0.55557024, 0.5657318,
    0.57580817, 0.58579785, 0.5956993, 0.60551107, 0.6152316,
    0.6248595, 0.6343933, 0.64383155, 0.65317285, 0.6624158, 0.671559,
    0.680601, 0.68954057, 0.69837624, 0.70710677, 0.71573085,
    0.7242471, 0.7326543, 0.7409511, 0.7491364, 0.7572088, 0.76516724,
    0.77301043, 0.7807372, 0.7883464, 0.7958369, 0.8032075,
    0.81045717, 0.8175848, 0.8245893, 0.8314696, 0.8382247, 0.8448536,
    0.8513552, 0.8577286, 0.86397284, 0.87008697, 0.8760701,
    0.8819213, 0.88763964, 0.8932243, 0.8986745, 0.9039893, 0.909168,
    0.9142098, 0.9191139, 0.9238795, 0.9285061, 0.9329928, 0.937339,
    0.94154406, 0.9456073, 0.94952816, 0.953306, 0.95694035,
    0.9604305, 0.96377605, 0.96697646, 0.97003126, 0.97293997,
    0.9757021, 0.9783174, 0.98078525, 0.9831055, 0.98527765,
    0.9873014, 0.9891765, 0.99090266, 0.99247956, 0.993907, 0.9951847,
    0.9963126, 0.99729043, 0.9981181, 0.99879545, 0.99932235,
    0.9996988, 0.9999247, 1., 0.9999247, 0.9996988, 0.99932235,
    0.99879545, 0.9981181, 0.99729043, 0.9963126, 0.9951847, 0.993907,
    0.99247956, 0.99090266, 0.9891765, 0.9873014, 0.98527765,
    0.9831055, 0.98078525, 0.9783174, 0.9757021, 0.97293997,
    0.97003126, 0.96697646, 0.96377605, 0.9604305, 0.95694035,
    0.953306, 0.94952816, 0.9456073, 0.94154406, 0.937339, 0.9329928,
    0.9285061, 0.9238795, 0.9191139, 0.9142098, 0.909168, 0.9039893,
    0.8986745, 0.8932243, 0.88763964, 0.8819213, 0.8760701,
    0.87008697, 0.86397284, 0.8577286, 0.8513552, 0.8448536,
    0.8382247,0.8314696, 0.8245893, 0.8175848, 0.81045717, 0.8032075,
    0.7958369, 0.7883464, 0.7807372, 0.77301043, 0.76516724,
    0.7572088, 0.7491364, 0.7409511, 0.7326543, 0.7242471, 0.71573085,
    0.70710677, 0.69837624, 0.68954057, 0.680601, 0.671559, 0.6624158,
    0.65317285, 0.64383155, 0.6343933, 0.6248595, 0.6152316,
    0.60551107, 0.5956993, 0.58579785, 0.57580817, 0.5657318,
    0.55557024, 0.545325, 0.53499764, 0.52458966, 0.51410276,
    0.50353837, 0.4928982, 0.48218378, 0.47139674, 0.46053872,
    0.44961134, 0.43861625, 0.42755508, 0.41642955, 0.4052413,
    0.39399204, 0.38268343, 0.3713172, 0.35989505, 0.34841868,
    0.33688986, 0.3253103, 0.31368175, 0.30200595, 0.29028466,
    0.2785197, 0.26671275, 0.25486565, 0.24298018, 0.2310581,
    0.21910124, 0.20711137, 0.19509032, 0.18303989, 0.17096189,
    0.15885815, 0.14673047, 0.1345807, 0.12241068, 0.110222206,
    0.09801714, 0.08579731, 0.07356457, 0.061320737, 0.049067676,
    0.036807224, 0.024541229, 0.012271538, 0.00000000000000012246469,
    -0.012271538, -0.024541229, -0.036807224, -0.049067676,
    -0.061320737, -0.07356457, -0.08579731, -0.09801714, -0.110222206,
    -0.12241068, -0.1345807, -0.14673047, -0.15885815, -0.17096189,
    -0.18303989, -0.19509032, -0.20711137, -0.21910124, -0.2310581,
    -0.24298018, -0.25486565, -0.26671275, -0.2785197, -0.29028466,
    -0.30200595, -0.31368175, -0.3253103, -0.33688986, -0.34841868,
    -0.35989505, -0.3713172, -0.38268343, -0.39399204, -0.4052413,
    -0.41642955, -0.42755508, -0.43861625, -0.44961134, -0.46053872,
    -0.47139674, -0.48218378, -0.4928982, -0.50353837, -0.51410276,
    -0.52458966, -0.53499764, -0.545325, -0.55557024, -0.5657318,
    -0.57580817, -0.58579785, -0.5956993, -0.60551107, -0.6152316,
    -0.6248595, -0.6343933, -0.64383155, -0.65317285, -0.6624158,
    -0.671559, -0.680601, -0.68954057, -0.69837624, -0.70710677,
    -0.71573085,-0.7242471, -0.7326543, -0.7409511, -0.7491364,
    -0.7572088, -0.76516724, -0.77301043, -0.7807372, -0.7883464,
    -0.7958369, -0.8032075, -0.81045717, -0.8175848, -0.8245893,
    -0.8314696, -0.8382247, -0.8448536,-0.8513552, -0.8577286,
    -0.86397284, -0.87008697, -0.8760701, -0.8819213, -0.88763964,
    -0.8932243, -0.8986745, -0.9039893, -0.909168, -0.9142098,
    -0.9191139, -0.9238795, -0.9285061, -0.9329928, -0.937339,
    -0.94154406, -0.9456073, -0.94952816, -0.953306, -0.95694035,
    -0.9604305, -0.96377605, -0.96697646, -0.97003126, -0.97293997,
    -0.9757021, -0.9783174, -0.98078525, -0.9831055, -0.98527765,
    -0.9873014, -0.9891765, -0.99090266, -0.99247956, -0.993907,
    -0.9951847, -0.9963126, -0.99729043, -0.9981181, -0.99879545,
    -0.99932235, -0.9996988, -0.9999247, -1., -0.9999247, -0.9996988,
    -0.99932235, -0.99879545, -0.9981181, -0.99729043, -0.9963126,
    -0.9951847, -0.993907, -0.99247956, -0.99090266, -0.9891765,
    -0.9873014, -0.98527765, -0.9831055, -0.98078525, -0.9783174,
    -0.9757021, -0.97293997, -0.97003126, -0.96697646, -0.96377605,
    -0.9604305, -0.95694035, -0.953306, -0.94952816, -0.9456073,
    -0.94154406, -0.937339, -0.9329928, -0.9285061, -0.9238795,
    -0.9191139, -0.9142098, -0.909168, -0.9039893, -0.8986745,
    -0.8932243, -0.88763964, -0.8819213, -0.8760701, -0.87008697,
    -0.86397284, -0.8577286, -0.8513552, -0.8448536, -0.8382247,
    -0.8314696, -0.8245893, -0.8175848, -0.81045717, -0.8032075,
    -0.7958369, -0.7883464, -0.7807372, -0.77301043, -0.76516724,
    -0.7572088, -0.7491364, -0.7409511, -0.7326543, -0.7242471,
    -0.71573085, -0.70710677, -0.69837624, -0.68954057,
    -0.680601,-0.671559, -0.6624158, -0.65317285, -0.64383155,
    -0.6343933, -0.6248595, -0.6152316, -0.60551107, -0.5956993,
    -0.58579785, -0.57580817, -0.5657318, -0.55557024, -0.545325,
    -0.53499764, -0.52458966, -0.51410276, -0.50353837, -0.4928982,
    -0.48218378, -0.47139674, -0.46053872, -0.44961134, -0.43861625,
    -0.42755508, -0.41642955, -0.4052413, -0.39399204, -0.38268343,
    -0.3713172, -0.35989505, -0.34841868, -0.33688986, -0.3253103,
    -0.31368175, -0.30200595, -0.29028466, -0.2785197, -0.26671275,
    -0.25486565, -0.24298018, -0.2310581, -0.21910124, -0.20711137,
    -0.19509032, -0.18303989, -0.17096189, -0.15885815,
    -0.14673047,-0.1345807, -0.12241068, -0.110222206, -0.09801714,
    -0.08579731, -0.07356457, -0.061320737, -0.049067676,
    -0.036807224, -0.024541229, -0.012271538
];

/*
pub fn generate_sin() {
    let length = 512;
    let range = 1.;
    let pi = std::f64::consts::PI;
    for x in 0..length {
        let phase = (x as f64 / length as f64) * pi * 2.;
        let y = phase.sin();
        print!("{}, ", (y * range) as f32);
    }
}
*/
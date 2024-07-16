/* w = [0..1> phase in Q0.16bit format (w = 1 ~ 2*pi)
 *     0b qqtt_tttt_tttt_iiii
 *     q = quadrant
 *     t = table entry index
 *     i = linear interpolation between t-th  and t+1 -th table entry
 */
const FRAC_BITS : i32 = 4;
const FRAC_LEN  : i32 = 1 << FRAC_BITS;
const FRAC_MASK : i32 = FRAC_LEN - 1;

const TABLE_BITS : i32 = 10;
const TABLE_LEN  : i32 = 1 << TABLE_BITS;
const TABLE_MASK : i32 = TABLE_LEN - 1;
const TABLE_SHIFT : i32 = FRAC_BITS;

/// Return sin(w) 
///
/// 'w' is phase - phase of 1 corresponds to an angle of 2.pi 
/// 'w' is interpreted as 16.16 fix-point value - but only the lowest 14 bits
///     are considered, reflecting a phase in range [0..0.25>
///
/// return value is [0..1> in 16.16 fixpoint
pub fn sin_from_table(w : i32) -> i32 {
    let idx : usize = (((w >> TABLE_SHIFT) & TABLE_MASK) as u16).into();
	let div : i32 = w & FRAC_MASK; 

	/* get values from table */
	let bot : i32 = SIN_TABLE[idx ] as i32;
	let top : i32 = SIN_TABLE[idx + 1] as i32;

	/* interpolate and return */
	(bot * (FRAC_LEN - div) + top * div) >> FRAC_BITS
}

const SIN_TABLE: [u16; (TABLE_LEN + 1) as usize] = [
	    0,   100,   201,   301,   402,   502,   603,   703,   804,   904,  1005,  1105,  1206,  1306,  1407,  1507, 
	 1608,  1708,  1809,  1909,  2010,  2110,  2211,  2311,  2412,  2512,  2613,  2713,  2814,  2914,  3014,  3115, 
	 3215,  3316,  3416,  3516,  3617,  3717,  3818,  3918,  4018,  4119,  4219,  4319,  4420,  4520,  4620,  4720, 
	 4821,  4921,  5021,  5121,  5222,  5322,  5422,  5522,  5622,  5722,  5823,  5923,  6023,  6123,  6223,  6323, 
	 6423,  6523,  6623,  6723,  6823,  6923,  7023,  7123,  7223,  7323,  7423,  7523,  7623,  7722,  7822,  7922, 
	 8022,  8122,  8221,  8321,  8421,  8520,  8620,  8720,  8819,  8919,  9019,  9118,  9218,  9317,  9417,  9516, 
	 9616,  9715,  9814,  9914, 10013, 10113, 10212, 10311, 10410, 10510, 10609, 10708, 10807, 10906, 11006, 11105, 
	11204, 11303, 11402, 11501, 11600, 11699, 11797, 11896, 11995, 12094, 12193, 12292, 12390, 12489, 12588, 12686, 
	12785, 12884, 12982, 13081, 13179, 13278, 13376, 13474, 13573, 13671, 13769, 13868, 13966, 14064, 14162, 14260, 
	14359, 14457, 14555, 14653, 14751, 14849, 14946, 15044, 15142, 15240, 15338, 15435, 15533, 15631, 15728, 15826, 
	15923, 16021, 16118, 16216, 16313, 16411, 16508, 16605, 16702, 16800, 16897, 16994, 17091, 17188, 17285, 17382, 
	17479, 17576, 17672, 17769, 17866, 17963, 18059, 18156, 18253, 18349, 18446, 18542, 18638, 18735, 18831, 18927, 
	19024, 19120, 19216, 19312, 19408, 19504, 19600, 19696, 19792, 19888, 19983, 20079, 20175, 20270, 20366, 20461, 
	20557, 20652, 20748, 20843, 20938, 21034, 21129, 21224, 21319, 21414, 21509, 21604, 21699, 21794, 21889, 21983, 
	22078, 22173, 22267, 22362, 22456, 22551, 22645, 22739, 22833, 22928, 23022, 23116, 23210, 23304, 23398, 23492, 
	23586, 23679, 23773, 23867, 23960, 24054, 24147, 24241, 24334, 24427, 24521, 24614, 24707, 24800, 24893, 24986, 
	25079, 25172, 25265, 25357, 25450, 25543, 25635, 25728, 25820, 25913, 26005, 26097, 26189, 26281, 26373, 26465, 
	26557, 26649, 26741, 26833, 26925, 27016, 27108, 27199, 27291, 27382, 27473, 27565, 27656, 27747, 27838, 27929, 
	28020, 28111, 28201, 28292, 28383, 28473, 28564, 28654, 28745, 28835, 28925, 29015, 29105, 29196, 29285, 29375, 
	29465, 29555, 29645, 29734, 29824, 29913, 30003, 30092, 30181, 30271, 30360, 30449, 30538, 30627, 30715, 30804, 
	30893, 30982, 31070, 31159, 31247, 31335, 31424, 31512, 31600, 31688, 31776, 31864, 31952, 32039, 32127, 32215, 
	32302, 32390, 32477, 32564, 32651, 32738, 32826, 32912, 32999, 33086, 33173, 33260, 33346, 33433, 33519, 33605, 
	33692, 33778, 33864, 33950, 34036, 34122, 34208, 34293, 34379, 34465, 34550, 34635, 34721, 34806, 34891, 34976, 
	35061, 35146, 35231, 35316, 35400, 35485, 35569, 35654, 35738, 35822, 35906, 35990, 36074, 36158, 36242, 36326, 
	36409, 36493, 36576, 36660, 36743, 36826, 36909, 36992, 37075, 37158, 37241, 37324, 37406, 37489, 37571, 37653, 
	37736, 37818, 37900, 37982, 38064, 38146, 38227, 38309, 38390, 38472, 38553, 38634, 38716, 38797, 38878, 38958, 
	39039, 39120, 39201, 39281, 39362, 39442, 39522, 39602, 39682, 39762, 39842, 39922, 40002, 40081, 40161, 40240, 
	40319, 40399, 40478, 40557, 40636, 40714, 40793, 40872, 40950, 41029, 41107, 41185, 41263, 41342, 41419, 41497, 
	41575, 41653, 41730, 41808, 41885, 41962, 42040, 42117, 42194, 42271, 42347, 42424, 42501, 42577, 42653, 42730, 
	42806, 42882, 42958, 43034, 43110, 43185, 43261, 43336, 43412, 43487, 43562, 43637, 43712, 43787, 43862, 43936, 
	44011, 44085, 44160, 44234, 44308, 44382, 44456, 44530, 44603, 44677, 44750, 44824, 44897, 44970, 45043, 45116, 
	45189, 45262, 45335, 45407, 45480, 45552, 45624, 45696, 45768, 45840, 45912, 45984, 46055, 46127, 46198, 46269, 
	46340, 46411, 46482, 46553, 46624, 46695, 46765, 46835, 46906, 46976, 47046, 47116, 47186, 47255, 47325, 47394, 
	47464, 47533, 47602, 47671, 47740, 47809, 47878, 47946, 48015, 48083, 48151, 48219, 48288, 48355, 48423, 48491, 
	48558, 48626, 48693, 48760, 48828, 48895, 48961, 49028, 49095, 49161, 49228, 49294, 49360, 49426, 49492, 49558, 
	49624, 49690, 49755, 49820, 49886, 49951, 50016, 50081, 50146, 50210, 50275, 50339, 50403, 50468, 50532, 50596, 
	50660, 50723, 50787, 50850, 50914, 50977, 51040, 51103, 51166, 51229, 51291, 51354, 51416, 51478, 51541, 51603, 
	51665, 51726, 51788, 51850, 51911, 51972, 52033, 52095, 52155, 52216, 52277, 52338, 52398, 52458, 52518, 52579, 
	52639, 52698, 52758, 52818, 52877, 52936, 52996, 53055, 53114, 53172, 53231, 53290, 53348, 53407, 53465, 53523, 
	53581, 53639, 53696, 53754, 53811, 53869, 53926, 53983, 54040, 54097, 54153, 54210, 54266, 54323, 54379, 54435, 
	54491, 54546, 54602, 54658, 54713, 54768, 54823, 54879, 54933, 54988, 55043, 55097, 55152, 55206, 55260, 55314, 
	55368, 55422, 55475, 55529, 55582, 55635, 55688, 55741, 55794, 55847, 55899, 55952, 56004, 56056, 56108, 56160, 
	56212, 56263, 56315, 56366, 56417, 56468, 56519, 56570, 56621, 56671, 56722, 56772, 56822, 56872, 56922, 56972, 
	57022, 57071, 57120, 57170, 57219, 57268, 57316, 57365, 57414, 57462, 57510, 57558, 57606, 57654, 57702, 57750, 
	57797, 57844, 57892, 57939, 57986, 58032, 58079, 58125, 58172, 58218, 58264, 58310, 58356, 58402, 58447, 58493, 
	58538, 58583, 58628, 58673, 58718, 58762, 58807, 58851, 58895, 58939, 58983, 59027, 59070, 59114, 59157, 59200, 
	59243, 59286, 59329, 59372, 59414, 59457, 59499, 59541, 59583, 59625, 59666, 59708, 59749, 59790, 59831, 59872, 
	59913, 59954, 59994, 60035, 60075, 60115, 60155, 60195, 60235, 60274, 60313, 60353, 60392, 60431, 60470, 60508, 
	60547, 60585, 60624, 60662, 60700, 60737, 60775, 60813, 60850, 60887, 60924, 60961, 60998, 61035, 61071, 61108, 
	61144, 61180, 61216, 61252, 61288, 61323, 61359, 61394, 61429, 61464, 61499, 61533, 61568, 61602, 61637, 61671, 
	61705, 61738, 61772, 61805, 61839, 61872, 61905, 61938, 61971, 62003, 62036, 62068, 62100, 62133, 62164, 62196, 
	62228, 62259, 62291, 62322, 62353, 62384, 62414, 62445, 62475, 62506, 62536, 62566, 62596, 62625, 62655, 62684, 
	62714, 62743, 62772, 62800, 62829, 62858, 62886, 62914, 62942, 62970, 62998, 63026, 63053, 63080, 63108, 63135, 
	63162, 63188, 63215, 63241, 63268, 63294, 63320, 63346, 63371, 63397, 63422, 63447, 63473, 63498, 63522, 63547, 
	63571, 63596, 63620, 63644, 63668, 63692, 63715, 63739, 63762, 63785, 63808, 63831, 63854, 63876, 63899, 63921, 
	63943, 63965, 63987, 64009, 64030, 64051, 64073, 64094, 64115, 64135, 64156, 64176, 64197, 64217, 64237, 64257, 
	64276, 64296, 64315, 64334, 64353, 64372, 64391, 64410, 64428, 64447, 64465, 64483, 64501, 64518, 64536, 64553, 
	64571, 64588, 64605, 64622, 64638, 64655, 64671, 64687, 64703, 64719, 64735, 64751, 64766, 64781, 64796, 64811, 
	64826, 64841, 64855, 64870, 64884, 64898, 64912, 64926, 64939, 64953, 64966, 64979, 64992, 65005, 65018, 65030, 
	65043, 65055, 65067, 65079, 65091, 65102, 65114, 65125, 65136, 65147, 65158, 65169, 65179, 65190, 65200, 65210, 
	65220, 65230, 65239, 65249, 65258, 65267, 65276, 65285, 65294, 65302, 65311, 65319, 65327, 65335, 65343, 65350, 
	65358, 65365, 65372, 65379, 65386, 65393, 65400, 65406, 65412, 65418, 65424, 65430, 65436, 65441, 65446, 65452, 
	65457, 65461, 65466, 65471, 65475, 65479, 65483, 65487, 65491, 65495, 65498, 65501, 65505, 65508, 65511, 65513, 
	65516, 65518, 65520, 65522, 65524, 65526, 65528, 65529, 65531, 65532, 65533, 65534, 65534, 65535, 65535, 65535, 
	65535 ];

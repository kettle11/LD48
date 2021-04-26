use crate::*;
pub(crate) fn get_level() -> Level {
    Level {
        things: vec![
            LevelItem {
                position: Vec2::new(1521.5884, 436.8792),
                half_size: Vec2::new(72.241516, 67.726425),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-5240.0, 4673.0),
                half_size: Vec2::new(4980.0, 4220.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(4460.0, 4683.0),
                half_size: Vec2::new(4139.9995, 4149.9995),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(31.034367, 538.51715),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-251.72415, 4803.548),
                half_size: Vec2::new(44.827553, 262.06885),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(106.06069, 4711.182),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-158.62068, 4975.962),
                half_size: Vec2::new(137.93102, 137.93091),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-41.37932, 5072.5137),
                half_size: Vec2::new(75.86216, 117.24121),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(266.00003, 5205.3765),
                half_size: Vec2::new(79.33332, 190.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-131.03455, 5179.41),
                half_size: Vec2::new(262.06897, 127.58594),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-151.72421, 5358.7207),
                half_size: Vec2::new(172.41364, 127.58618),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(99.999985, 5375.3765),
                half_size: Vec2::new(50.666653, 42.666504),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(244.82767, 5544.9277),
                half_size: Vec2::new(134.48267, 106.89673),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(306.5, 5855.5415),
                half_size: Vec2::new(31.499985, 221.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-212.49997, 5762.169),
                half_size: Vec2::new(87.50009, 112.499756),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(43.499992, 5781.5415),
                half_size: Vec2::new(61.5, 49.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(68.75009, 6049.669),
                half_size: Vec2::new(81.24995, 274.99976),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-228.5, 5914.0415),
                half_size: Vec2::new(59.5, 65.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-3.4999962, 6070.5415),
                half_size: Vec2::new(20.500011, 204.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-111.000015, 6308.5415),
                half_size: Vec2::new(17.0, 23.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-193.74997, 6437.1694),
                half_size: Vec2::new(81.24994, 125.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(318.74985, 6612.169),
                half_size: Vec2::new(68.750084, 250.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(42.105522, 6468.3545),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-198.93625, 6833.8506),
                half_size: Vec2::new(13.829826, 225.53198),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-252.12775, 6695.5527),
                half_size: Vec2::new(13.829826, 44.68091),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-207.44691, 6707.255),
                half_size: Vec2::new(13.829788, 43.616943),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-104.25537, 6776.4043),
                half_size: Vec2::new(85.106415, 82.97876),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(79.78725, 6860.447),
                half_size: Vec2::new(39.361717, 118.08496),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(136.17027, 6922.149),
                half_size: Vec2::new(46.808514, 81.91504),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(273.61716, 6958.1055),
                half_size: Vec2::new(20.00003, 28.08496),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(207.4469, 6997.6807),
                half_size: Vec2::new(81.91489, 46.80835),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(50.51544, 7081.179),
                half_size: Vec2::new(248.45337, 85.566895),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(59.793747, 7204.8906),
                half_size: Vec2::new(204.1235, 64.94824),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(30.92781, 7355.4063),
                half_size: Vec2::new(43.298935, 44.329834),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-216.49461, 7369.839),
                half_size: Vec2::new(20.618515, 19.587646),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(277.3193, 7379.117),
                half_size: Vec2::new(25.773155, 20.618408),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-139.17511, 7439.942),
                half_size: Vec2::new(128.86581, 36.08252),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-5.154682, 7434.787),
                half_size: Vec2::new(23.7113, 22.68042),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(209.27814, 7442.004),
                half_size: Vec2::new(120.61844, 27.83496),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(83.50506, 7446.1274),
                half_size: Vec2::new(31.958727, 11.340332),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(202.06165, 7529.633),
                half_size: Vec2::new(160.82462, 72.164795),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-152.57716, 7532.725),
                half_size: Vec2::new(154.63899, 67.010254),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-128.5, 7810.5664),
                half_size: Vec2::new(143.49997, 264.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(219.49998, 7687.0664),
                half_size: Vec2::new(181.5, 108.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(198.14812, 8209.967),
                half_size: Vec2::new(157.40733, 419.44434),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-141.66664, 8266.448),
                half_size: Vec2::new(152.77776, 331.48145),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-217.64705, 9097.219),
                half_size: Vec2::new(211.76466, 622.0591),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(330.8823, 8889.867),
                half_size: Vec2::new(307.35294, 270.58838),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1500.0001, 9580.203),
                half_size: Vec2::new(1028.5714, 864.28564),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-5211.923, 9561.162),
                half_size: Vec2::new(4908.077, 702.6929),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(5.6737924, 9058.287),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(568.81726, 9591.416),
                half_size: Vec2::new(527.95703, 441.93555),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-61.40352, 9814.002),
                half_size: Vec2::new(82.45616, 76.02344),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-218.71352, 9781.838),
                half_size: Vec2::new(45.61406, 29.824707),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-231.95862, 9832.775),
                half_size: Vec2::new(48.45356, 48.453613),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-57.89475, 9852.599),
                half_size: Vec2::new(53.216396, 65.49707),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-232.16379, 9892.949),
                half_size: Vec2::new(29.824577, 32.163574),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(431.0, 10076.577),
                half_size: Vec2::new(26.99997, 28.5),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(309.99997, 10074.077),
                half_size: Vec2::new(25.0, 25.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(367.0, 10139.577),
                half_size: Vec2::new(28.999985, 32.5),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-501.5747, 10384.738),
                half_size: Vec2::new(241.73224, 211.81104),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(99.21255, 10257.178),
                half_size: Vec2::new(77.16534, 77.95264),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(23.404247, 10284.244),
                half_size: Vec2::new(25.95744, 84.25537),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-71.063835, 10286.798),
                half_size: Vec2::new(26.808508, 32.34082),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-64.68087, 10318.713),
                half_size: Vec2::new(39.14894, 21.702148),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(58.267704, 10439.068),
                half_size: Vec2::new(55.118134, 132.2832),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(930.9851, 10752.131),
                half_size: Vec2::new(483.09814, 432.39404),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-123.62202, 10357.966),
                half_size: Vec2::new(57.480278, 37.007813),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-127.55904, 10466.627),
                half_size: Vec2::new(88.188965, 92.12598),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(163.3802, 10708.47),
                half_size: Vec2::new(174.64777, 222.53516),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-125.352, 10616.92),
                half_size: Vec2::new(69.01397, 94.36572),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-263.38, 10676.076),
                half_size: Vec2::new(43.661934, 97.183105),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-899.99927, 10926.779),
                half_size: Vec2::new(604.22485, 333.80273),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-61.97171, 10776.075),
                half_size: Vec2::new(174.64775, 36.620117),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-129.57736, 11038.047),
                half_size: Vec2::new(270.42224, 278.87305),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(392.07904, 10830.304),
                half_size: Vec2::new(31.683151, 20.791992),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(846.53455, 11466.9375),
                half_size: Vec2::new(292.07913, 441.58398),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1138.3333, 11833.871),
                half_size: Vec2::new(504.99997, 706.6665),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-648.5148, 11519.412),
                half_size: Vec2::new(161.38612, 389.1089),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(296.3857, 11137.726),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-132.5843, 11333.422),
                half_size: Vec2::new(69.6629, 44.943848),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(341.5841, 11326.344),
                half_size: Vec2::new(161.3861, 17.821777),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(389.10876, 11405.551),
                half_size: Vec2::new(92.07921, 77.22803),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-344.4443, 11422.598),
                half_size: Vec2::new(50.0, 50.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(248.31467, 11466.006),
                half_size: Vec2::new(30.33712, 35.95508),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(500.99002, 11660.996),
                half_size: Vec2::new(23.76242, 211.88135),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-468.57156, 11576.728),
                half_size: Vec2::new(51.42856, 107.61914),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(171.9101, 11501.961),
                half_size: Vec2::new(21.348305, 24.719238),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(381.1765, 11582.106),
                half_size: Vec2::new(70.58827, 71.76465),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-227.77771, 11587.875),
                half_size: Vec2::new(38.888847, 34.722168),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(223.5956, 11585.107),
                half_size: Vec2::new(21.348297, 24.719238),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-216.40623, 11819.979),
                half_size: Vec2::new(92.968735, 179.6875),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(372.54913, 11747.705),
                half_size: Vec2::new(35.294113, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(148.67253, 11873.116),
                half_size: Vec2::new(51.327465, 84.956055),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(261.94693, 11881.081),
                half_size: Vec2::new(51.327385, 85.84082),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1196.6667, 12170.537),
                half_size: Vec2::new(636.6666, 283.3335),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-576.92303, 11968.896),
                half_size: Vec2::new(34.615356, 36.538574),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-316.98114, 11990.232),
                half_size: Vec2::new(54.716965, 22.641602),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(68.867874, 12107.213),
                half_size: Vec2::new(61.320744, 49.05664),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(378.84613, 12134.282),
                half_size: Vec2::new(90.3846, 61.538086),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-469.5312, 12169.979),
                half_size: Vec2::new(88.281235, 96.875),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-40.384552, 12230.436),
                half_size: Vec2::new(151.92303, 123.07666),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(503.75003, 12302.165),
                half_size: Vec2::new(33.75003, 35.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-497.22205, 12317.042),
                half_size: Vec2::new(27.777786, 25.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-334.84833, 12448.149),
                half_size: Vec2::new(74.242386, 93.93945),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-493.93915, 12437.543),
                half_size: Vec2::new(54.545456, 71.2124),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(803.73816, 12905.667),
                half_size: Vec2::new(497.19623, 536.44824),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-847.2219, 12690.652),
                half_size: Vec2::new(155.55554, 262.4995),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-84.7222, 13037.875),
                half_size: Vec2::new(270.8332, 387.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(305.6074, 13056.135),
                half_size: Vec2::new(92.52334, 402.80322),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-601.3886, 13042.041),
                half_size: Vec2::new(223.61101, 377.77783),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-756.097, 13704.846),
                half_size: Vec2::new(243.90228, 465.85352),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(775.60913, 13851.1875),
                half_size: Vec2::new(463.41428, 563.41406),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(201.98047, 13315.078),
                half_size: Vec2::new(9.901039, 14.851074),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-130.95236, 13463.038),
                half_size: Vec2::new(8.730148, 54.76172),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-92.85709, 13455.102),
                half_size: Vec2::new(37.301582, 8.730469),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-168.25394, 13486.848),
                half_size: Vec2::new(39.682556, 10.317383),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-194.4444, 13517.006),
                half_size: Vec2::new(8.730141, 27.777832),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-221.42853, 13526.53),
                half_size: Vec2::new(29.36509, 8.730469),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-245.83319, 13929.54),
                half_size: Vec2::new(37.50004, 204.16699),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-47.916645, 13942.04),
                half_size: Vec2::new(52.083286, 191.66699),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-408.7301, 13826.53),
                half_size: Vec2::new(8.730148, 54.76172),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-152.17389, 13773.22),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::TorpedoUpgrade,
            },
            LevelItem {
                position: Vec2::new(-476.98404, 13932.086),
                half_size: Vec2::new(10.317474, 77.77783),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-427.7777, 13885.26),
                half_size: Vec2::new(45.238113, 5.555176),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(166.30438, 14025.754),
                half_size: Vec2::new(7.60865, 92.39111),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(190.21738, 13967.059),
                half_size: Vec2::new(20.652153, 7.6088867),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-404.76184, 14063.832),
                half_size: Vec2::new(19.047607, 87.30127),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-435.71417, 14001.927),
                half_size: Vec2::new(32.539658, 12.698242),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-679.7754, 14373.305),
                half_size: Vec2::new(153.93259, 324.71924),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(160.4166, 14171.207),
                half_size: Vec2::new(227.08322, 83.333496),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-441.66632, 14169.123),
                half_size: Vec2::new(195.83324, 77.08301),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-956.9236, 14359.891),
                half_size: Vec2::new(249.23096, 229.23096),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-40.44941, 14272.18),
                half_size: Vec2::new(62.921394, 75.28076),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-164.58337, 14210.139),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(207.86517, 14337.35),
                half_size: Vec2::new(225.8427, 91.01123),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1577.2723, 16019.486),
                half_size: Vec2::new(1231.8181, 1768.1816),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-234.83148, 14340.72),
                half_size: Vec2::new(140.44946, 56.179688),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1098.0007, 14848.967),
                half_size: Vec2::new(498.0003, 478.0005),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-115.714386, 14530.14),
                half_size: Vec2::new(272.85724, 157.14258),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(273.41772, 14491.068),
                half_size: Vec2::new(141.77217, 105.06348),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(306.06067, 14688.422),
                half_size: Vec2::new(118.18184, 133.3335),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-621.3484, 14741.844),
                half_size: Vec2::new(91.01123, 102.24756),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(50.90914, 14726.504),
                half_size: Vec2::new(14.545385, 20.000488),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(116.36367, 14735.594),
                half_size: Vec2::new(14.545387, 18.182129),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-3.6364212, 14741.049),
                half_size: Vec2::new(18.181805, 20.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-147.27267, 14864.686),
                half_size: Vec2::new(241.81827, 92.72705),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(334.0003, 14990.968),
                half_size: Vec2::new(150.00014, 204.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1412.2809, 15674.91),
                half_size: Vec2::new(626.3159, 805.2632),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-604.00037, 15028.968),
                half_size: Vec2::new(116.00012, 134.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-345.45465, 14931.958),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-150.9091, 15061.049),
                half_size: Vec2::new(194.54555, 129.09082),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-150.90909, 15279.23),
                half_size: Vec2::new(190.90921, 212.72705),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-699.99976, 15454.801),
                half_size: Vec2::new(130.76913, 317.94873),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(372.2224, 15365.695),
                half_size: Vec2::new(150.00005, 192.59277),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-600.00037, 15326.968),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-408.6206, 15391.273),
                half_size: Vec2::new(46.551758, 53.448242),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(124.27185, 15389.515),
                half_size: Vec2::new(27.18449, 33.009766),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(475.47162, 15893.844),
                half_size: Vec2::new(237.73581, 288.6792),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-747.3686, 16046.84),
                half_size: Vec2::new(143.85968, 415.78906),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-216.6667, 15762.92),
                half_size: Vec2::new(42.592606, 42.592285),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(61.90483, 15838.318),
                half_size: Vec2::new(66.66671, 85.714355),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-525.6412, 15929.16),
                half_size: Vec2::new(7.692322, 33.333496),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-544.872, 15917.622),
                half_size: Vec2::new(21.79486, 6.4101563),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-574.35925, 15957.365),
                half_size: Vec2::new(66.66672, 12.820801),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-211.11108, 15999.957),
                half_size: Vec2::new(33.33332, 35.185547),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(382.0, 16606.033),
                half_size: Vec2::new(217.99988, 506.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(16.666676, 16257.365),
                half_size: Vec2::new(64.28567, 71.42871),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-203.70372, 16262.92),
                half_size: Vec2::new(33.333313, 42.592773),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-459.99994, 16364.032),
                half_size: Vec2::new(68.8889, 73.33301),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-800.0, 16690.033),
                half_size: Vec2::new(195.99997, 254.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-144.00006, 16810.033),
                half_size: Vec2::new(127.99999, 302.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-451.99997, 16832.033),
                half_size: Vec2::new(227.99997, 232.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-783.33374, 17115.72),
                half_size: Vec2::new(133.3334, 250.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(79.54541, 17049.098),
                half_size: Vec2::new(61.363693, 29.545898),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-344.1174, 17268.588),
                half_size: Vec2::new(144.11755, 35.29492),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(167.64708, 17333.293),
                half_size: Vec2::new(91.17644, 35.293945),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-135.48387, 17788.174),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-1811.7637, 19327.41),
                half_size: Vec2::new(70.588196, 70.58789),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(652.9409, 19483.293),
                half_size: Vec2::new(41.17624, 55.882813),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1061.7642, 19489.176),
                half_size: Vec2::new(49.99994, 61.76465),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(814.7054, 19497.998),
                half_size: Vec2::new(49.99994, 47.058594),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(512.9032, 19513.55),
                half_size: Vec2::new(32.25818, 37.09668),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1779.4109, 19553.88),
                half_size: Vec2::new(44.117676, 44.117188),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1317.6472, 19550.035),
                half_size: Vec2::new(41.176453, 35.293945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1203.2257, 19600.648),
                half_size: Vec2::new(9.67749, 14.515625),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(477.41937, 19649.035),
                half_size: Vec2::new(22.580658, 20.967773),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(741.17615, 19727.41),
                half_size: Vec2::new(64.70599, 47.058594),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1120.5884, 19741.21),
                half_size: Vec2::new(44.117676, 44.118164),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(425.8065, 19734.52),
                half_size: Vec2::new(22.580658, 19.355469),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1314.706, 19761.797),
                half_size: Vec2::new(61.76471, 41.176758),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1711.7639, 19759.762),
                half_size: Vec2::new(47.058777, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-898.3871, 19792.584),
                half_size: Vec2::new(27.419342, 32.257813),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-150.00015, 19784.873),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(1026.4701, 19856.822),
                half_size: Vec2::new(50.000214, 47.058594),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1017.6472, 19873.563),
                half_size: Vec2::new(41.176514, 35.293945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1170.5884, 19894.152),
                half_size: Vec2::new(35.29419, 26.469727),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-840.3225, 19900.648),
                half_size: Vec2::new(27.419342, 24.19336),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(320.96765, 19937.746),
                half_size: Vec2::new(43.54843, 32.25879),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(638.2352, 20011.797),
                half_size: Vec2::new(49.99997, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1505.8816, 20030.352),
                half_size: Vec2::new(52.9411, 44.117188),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1123.5295, 20058.855),
                half_size: Vec2::new(47.058838, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(864.5161, 20079.68),
                half_size: Vec2::new(35.483948, 32.25879),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-194.11763, 20264.738),
                half_size: Vec2::new(47.058823, 44.118164),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(656.45166, 20278.066),
                half_size: Vec2::new(43.54843, 43.54785),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-388.23535, 20297.094),
                half_size: Vec2::new(64.70592, 47.05957),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-788.2354, 20311.799),
                half_size: Vec2::new(52.941162, 55.882813),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1038.2347, 20350.94),
                half_size: Vec2::new(102.94107, 70.58789),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-576.4706, 20332.387),
                half_size: Vec2::new(52.941162, 47.058594),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1590.3225, 20350.648),
                half_size: Vec2::new(29.032288, 25.80664),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1195.1613, 20400.648),
                half_size: Vec2::new(30.645142, 30.644531),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-341.17633, 20977.41),
                half_size: Vec2::new(17.64708, 455.88184),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(533.871, 20557.1),
                half_size: Vec2::new(33.870926, 32.257813),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-993.54834, 20590.973),
                half_size: Vec2::new(35.483917, 37.09668),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(879.4114, 20733.293),
                half_size: Vec2::new(67.64691, 76.46973),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-394.1175, 20815.645),
                half_size: Vec2::new(11.764755, 123.5293),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1461.764, 20762.703),
                half_size: Vec2::new(61.76477, 52.941406),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-438.2351, 20774.469),
                half_size: Vec2::new(38.2352, 17.646484),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-199.99995, 20871.527),
                half_size: Vec2::new(11.764755, 55.882813),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-288.23523, 20918.586),
                half_size: Vec2::new(111.76468, 20.58789),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1285.2935, 20977.41),
                half_size: Vec2::new(44.117493, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1711.7639, 20995.057),
                half_size: Vec2::new(52.9411, 50.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-670.5879, 21024.469),
                half_size: Vec2::new(35.294037, 20.58789),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1064.7053, 21077.41),
                half_size: Vec2::new(64.70575, 50.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-255.88223, 21071.527),
                half_size: Vec2::new(91.17632, 20.58789),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-14.705795, 21130.352),
                half_size: Vec2::new(79.411804, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-444.11743, 21300.938),
                half_size: Vec2::new(8.823608, 61.76465),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1802.9402, 21289.174),
                half_size: Vec2::new(55.882385, 44.117188),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(794.11725, 21353.879),
                half_size: Vec2::new(88.23505, 85.293945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-429.4115, 21362.703),
                half_size: Vec2::new(82.35295, 17.646484),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1144.117, 21436.234),
                half_size: Vec2::new(79.411804, 85.293945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1464.7052, 21500.938),
                half_size: Vec2::new(70.58826, 55.881836),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-326.47046, 21915.645),
                half_size: Vec2::new(14.705795, 435.29395),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-367.6469, 21550.938),
                half_size: Vec2::new(32.352997, 29.411133),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-120.58816, 21659.762),
                half_size: Vec2::new(14.705925, 44.117188),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-197.0588, 21703.879),
                half_size: Vec2::new(114.70584, 11.764648),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(935.2937, 21818.586),
                half_size: Vec2::new(41.17624, 38.23535),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1223.5288, 21824.469),
                half_size: Vec2::new(41.176453, 32.35254),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-566.6661, 21889.313),
                half_size: Vec2::new(14.285828, 80.953125),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1670.5874, 21886.232),
                half_size: Vec2::new(52.9411, 29.41211),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(444.1173, 21912.703),
                half_size: Vec2::new(67.64691, 38.234375),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-466.6661, 21955.98),
                half_size: Vec2::new(133.33328, 14.286133),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(480.9525, 22591.82),
                half_size: Vec2::new(47.619003, 535.71387),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(333.33344, 22298.965),
                half_size: Vec2::new(52.38089, 214.28613),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-953.57153, 22162.285),
                half_size: Vec2::new(53.57144, 53.57129),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(230.95242, 22518.012),
                half_size: Vec2::new(69.047676, 338.0957),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(773.0769, 22657.938),
                half_size: Vec2::new(165.38467, 423.07715),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(659.52405, 22587.059),
                half_size: Vec2::new(145.23816, 288.09473),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-660.7146, 22558.49),
                half_size: Vec2::new(132.14297, 250.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(135.71454, 22737.063),
                half_size: Vec2::new(35.71441, 407.14355),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-326.15393, 22731.785),
                half_size: Vec2::new(15.384598, 387.69238),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-817.85754, 22701.348),
                half_size: Vec2::new(39.285675, 107.14258),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(403.22595, 22622.621),
                half_size: Vec2::new(18.279572, 22.581055),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-714.286, 22854.918),
                half_size: Vec2::new(292.8574, 175.00098),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-180.39226, 22862.883),
                half_size: Vec2::new(5.88237, 39.21582),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-230.39227, 22918.766),
                half_size: Vec2::new(6.862747, 50.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(90.90918, 23317.906),
                half_size: Vec2::new(54.545353, 425.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-176.4707, 22907.98),
                half_size: Vec2::new(50.980446, 7.8427734),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-138.23535, 22955.04),
                half_size: Vec2::new(6.862755, 39.21582),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-255.93231, 22924.781),
                half_size: Vec2::new(23.728844, 4.2373047),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-188.13568, 22948.51),
                half_size: Vec2::new(8.474609, 9.322266),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-115.686356, 22949.158),
                half_size: Vec2::new(17.647099, 5.8828125),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-646.42896, 23072.777),
                half_size: Vec2::new(475.00027, 107.143555),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-135.17252, 22994.592),
                half_size: Vec2::new(15.172428, 13.103516),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-128.27234, 23071.703),
                half_size: Vec2::new(6.8062706, 81.674805),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(554.5459, 23311.086),
                half_size: Vec2::new(251.51535, 290.90918),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-141.17654, 23064.844),
                half_size: Vec2::new(15.686287, 5.881836),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-715.3862, 23334.863),
                half_size: Vec2::new(469.23184, 230.77051),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-99.019684, 23118.766),
                half_size: Vec2::new(22.549038, 6.8632813),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-125.13097, 23206.258),
                half_size: Vec2::new(4.712063, 58.115234),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-162.82733, 23194.738),
                half_size: Vec2::new(2.617813, 22.512695),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-163.72559, 23221.707),
                half_size: Vec2::new(34.313766, 5.8828125),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-145.1329, 23271.645),
                half_size: Vec2::new(3.5398483, 44.248047),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1071.4287, 24532.297),
                half_size: Vec2::new(100.00006, 1128.5713),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-737.77704, 23706.5),
                half_size: Vec2::new(137.77765, 262.22168),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1130.0007, 24485.633),
                half_size: Vec2::new(649.9999, 1020.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(822.727, 23579.268),
                half_size: Vec2::new(177.27289, 104.54492),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-351.5788, 23562.367),
                half_size: Vec2::new(37.8947, 54.737305),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-964.70654, 23599.158),
                half_size: Vec2::new(68.62747, 81.37305),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-583.33307, 23598.688),
                half_size: Vec2::new(62.121216, 58.333984),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-310.52618, 23638.156),
                half_size: Vec2::new(28.421036, 37.895508),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-412.63138, 23621.313),
                half_size: Vec2::new(40.0, 18.947266),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-404.87814, 23669.531),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(84.962364, 23727.781),
                half_size: Vec2::new(81.954865, 43.609375),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(208.33328, 23732.78),
                half_size: Vec2::new(44.696968, 12.121094),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(6.0150347, 23762.367),
                half_size: Vec2::new(12.030101, 39.097656),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-942.8578, 23813.543),
                half_size: Vec2::new(59.34076, 56.043945),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-295.48877, 23808.979),
                half_size: Vec2::new(9.774445, 12.029297),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-245.86472, 23817.25),
                half_size: Vec2::new(11.278206, 11.27832),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-270.67673, 23844.316),
                half_size: Vec2::new(22.556396, 30.827148),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(93.98498, 23868.375),
                half_size: Vec2::new(9.774384, 9.774414),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-560.90234, 23866.121),
                half_size: Vec2::new(7.5188293, 7.5195313),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(140.60147, 23878.902),
                half_size: Vec2::new(12.781956, 14.286133),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-783.33307, 23873.965),
                half_size: Vec2::new(225.0, 8.333008),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-214.75409, 23881.37),
                half_size: Vec2::new(313.11475, 9.8359375),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-558.33307, 23882.297),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(340.98358, 23901.04),
                half_size: Vec2::new(199.99991, 16.393555),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-531.57904, 23896.195),
                half_size: Vec2::new(11.278198, 9.022461),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-351.87973, 23907.475),
                half_size: Vec2::new(6.015045, 6.767578),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-32.075504, 23911.795),
                half_size: Vec2::new(6.918232, 6.9179688),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-469.17297, 23911.984),
                half_size: Vec2::new(6.015045, 6.7666016),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-151.12782, 23912.738),
                half_size: Vec2::new(8.27066, 7.5185547),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(81.9549, 23911.986),
                half_size: Vec2::new(5.263191, 5.263672),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-91.729324, 23910.482),
                half_size: Vec2::new(4.5112877, 3.7597656),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-260.15036, 23914.242),
                half_size: Vec2::new(4.511284, 4.510742),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-303.00757, 23917.25),
                half_size: Vec2::new(5.2631683, 6.0146484),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-208.25694, 23920.123),
                half_size: Vec2::new(6.4220276, 6.421875),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(481.95496, 23926.271),
                half_size: Vec2::new(11.278198, 12.029297),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-411.27823, 23927.775),
                half_size: Vec2::new(11.278168, 9.0234375),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-644.2623, 23940.385),
                half_size: Vec2::new(234.42624, 13.115234),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(28.301912, 23935.693),
                half_size: Vec2::new(8.176115, 6.9179688),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-96.22649, 23936.324),
                half_size: Vec2::new(10.691826, 7.5478516),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(232.33081, 23936.797),
                half_size: Vec2::new(8.270699, 7.5185547),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(120.30083, 23936.797),
                half_size: Vec2::new(12.030079, 6.0146484),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-247.36844, 23937.55),
                half_size: Vec2::new(5.2631607, 3.758789),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-336.8421, 23950.332),
                half_size: Vec2::new(13.533829, 12.030273),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(14.754089, 23968.254),
                half_size: Vec2::new(352.459, 18.033203),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(364.6617, 23989.43),
                half_size: Vec2::new(11.278198, 10.526367),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-630.07526, 24002.21),
                half_size: Vec2::new(12.03009, 12.782227),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-857.14343, 24053.102),
                half_size: Vec2::new(186.81335, 62.637695),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-247.25293, 24022.332),
                half_size: Vec2::new(385.71454, 20.878906),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(338.46173, 24131.125),
                half_size: Vec2::new(167.03304, 101.09863),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(133.0827, 24043.566),
                half_size: Vec2::new(11.278198, 12.030273),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-460.9378, 24156.879),
                half_size: Vec2::new(14.0625, 112.5),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-872.8814, 24362.922),
                half_size: Vec2::new(177.96613, 249.15332),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(63.076942, 24271.781),
                half_size: Vec2::new(75.384705, 64.615234),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-662.5001, 24319.377),
                half_size: Vec2::new(54.99997, 61.25),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(281.35602, 24276.48),
                half_size: Vec2::new(138.98303, 10.169922),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(418.18182, 24308.96),
                half_size: Vec2::new(12.121277, 28.788086),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(273.68433, 24378.262),
                half_size: Vec2::new(142.1052, 94.737305),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(423.72888, 24520.547),
                half_size: Vec2::new(10.169449, 227.11816),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(281.57904, 24515.104),
                half_size: Vec2::new(134.21051, 178.94727),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(456.71667, 24419.66),
                half_size: Vec2::new(11.940353, 20.895508),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-141.02568, 24748.707),
                half_size: Vec2::new(217.9488, 235.89746),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-916.66675, 25325.629),
                half_size: Vec2::new(116.66675, 750.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-797.77747, 24695.578),
                half_size: Vec2::new(40.740723, 104.444336),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(415.78955, 25088.787),
                half_size: Vec2::new(10.526276, 263.1582),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-751.11084, 25177.063),
                half_size: Vec2::new(84.44446, 311.85254),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-621.48126, 25080.023),
                half_size: Vec2::new(9.629639, 185.18457),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-649.55804, 24994.621),
                half_size: Vec2::new(8.849548, 11.503906),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-341.5932, 25209.664),
                half_size: Vec2::new(5.3097534, 23.00879),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-323.89404, 25206.125),
                half_size: Vec2::new(15.929199, 5.3095703),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-315.92944, 25232.676),
                half_size: Vec2::new(4.4248047, 23.00879),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-439.4737, 25244.05),
                half_size: Vec2::new(97.368454, 18.420898),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(281.72046, 25347.86),
                half_size: Vec2::new(8.602188, 112.90332),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-300.00024, 25247.719),
                half_size: Vec2::new(11.50441, 6.1953125),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-623.6558, 25280.117),
                half_size: Vec2::new(10.752686, 32.257813),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-294.6905, 25275.152),
                half_size: Vec2::new(2.6548462, 24.779297),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(62.83187, 25280.46),
                half_size: Vec2::new(4.424759, 28.319336),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(86.72572, 25266.303),
                half_size: Vec2::new(3.5398064, 7.080078),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(81.41593, 25269.844),
                half_size: Vec2::new(14.159302, 5.3095703),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(45.132763, 25277.807),
                half_size: Vec2::new(6.1947365, 7.9648438),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(261.38635, 25278.488),
                half_size: Vec2::new(19.801971, 4.951172),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(49.557518, 25285.771),
                half_size: Vec2::new(12.389399, 3.5390625),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(257.4805, 25296.86),
                half_size: Vec2::new(3.9369888, 13.385742),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-306.19495, 25293.734),
                half_size: Vec2::new(12.389404, 4.4248047),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-311.5047, 25309.664),
                half_size: Vec2::new(5.309738, 15.043945),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-594.62354, 25326.355),
                half_size: Vec2::new(41.935486, 29.032227),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(248.8191, 25302.371),
                half_size: Vec2::new(9.448776, 3.149414),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(102.222145, 25321.506),
                half_size: Vec2::new(48.888863, 13.333984),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-515.0537, 25401.621),
                half_size: Vec2::new(35.483856, 61.29004),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(269.89246, 25400.547),
                half_size: Vec2::new(46.23649, 53.76367),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-415.9295, 25369.844),
                half_size: Vec2::new(15.929245, 15.043945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-681.5791, 25570.367),
                half_size: Vec2::new(65.78946, 186.84277),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-28.947388, 25475.63),
                half_size: Vec2::new(518.42114, 65.78906),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1014.2857, 25829.531),
                half_size: Vec2::new(747.6189, 304.76172),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-886.84186, 26030.785),
                half_size: Vec2::new(376.31573, 389.47363),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-130.09705, 26045.813),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(1209.091, 26913.082),
                half_size: Vec2::new(936.36365, 836.36426),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-900.0, 26808.535),
                half_size: Vec2::new(381.81827, 650.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-990.0002, 28147.627),
                half_size: Vec2::new(429.99982, 920.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1349.9995, 28427.627),
                half_size: Vec2::new(1009.9997, 920.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1550.0001, 29077.627),
                half_size: Vec2::new(649.9999, 670.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1880.0, 29447.627),
                half_size: Vec2::new(999.9997, 700.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-1040.0, 30609.771),
                half_size: Vec2::new(599.99976, 1620.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-2700.0, 30123.0),
                half_size: Vec2::new(1280.0, 870.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(2550.0, 30903.0),
                half_size: Vec2::new(1089.9993, 1170.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(1199.9995, 31017.12),
                half_size: Vec2::new(384.61533, 984.61523),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(-569.9999, 31089.771),
                half_size: Vec2::new(230.00003, 740.0),
                thing_type: ThingType::Rock,
            },
            LevelItem {
                position: Vec2::new(221.739, 30355.338),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(239.13, 31390.12),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(3500.0, 32279.771),
                half_size: Vec2::new(59.999634, 70.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1568.9656, 32256.914),
                half_size: Vec2::new(37.93097, 41.379883),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(265.21695, 32277.076),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-665.5172, 32367.26),
                half_size: Vec2::new(79.3103, 68.96484),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2186.207, 32453.467),
                half_size: Vec2::new(68.965576, 65.51758),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1658.6206, 32453.467),
                half_size: Vec2::new(65.51727, 65.51758),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-2490.0, 32489.771),
                half_size: Vec2::new(89.99988, 80.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3260.0005, 32589.771),
                half_size: Vec2::new(160.0, 120.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1179.3103, 32591.398),
                half_size: Vec2::new(137.93097, 120.68945),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-917.24133, 32618.984),
                half_size: Vec2::new(68.965576, 86.20703),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3879.9995, 32639.771),
                half_size: Vec2::new(59.999634, 90.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2017.2413, 32612.088),
                half_size: Vec2::new(44.827515, 37.93164),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-3090.0, 32799.77),
                half_size: Vec2::new(149.99963, 109.99902),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(273.9129, 32720.555),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(3180.0, 32909.77),
                half_size: Vec2::new(39.999756, 60.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-803.44824, 32922.434),
                half_size: Vec2::new(58.620697, 58.621094),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1503.4485, 32936.227),
                half_size: Vec2::new(55.172424, 37.93164),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2641.3794, 33018.984),
                half_size: Vec2::new(55.172363, 51.722656),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3500.0, 33059.77),
                half_size: Vec2::new(99.99951, 90.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3950.0, 33059.77),
                half_size: Vec2::new(110.000244, 90.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1696.5518, 33067.258),
                half_size: Vec2::new(55.172363, 44.82617),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1986.2069, 33098.297),
                half_size: Vec2::new(48.276123, 48.27539),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-941.3794, 33098.297),
                half_size: Vec2::new(51.72409, 41.38086),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1559.9998, 33159.77),
                half_size: Vec2::new(99.99988, 90.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2451.724, 33112.086),
                half_size: Vec2::new(31.034668, 41.38086),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2279.3105, 33105.19),
                half_size: Vec2::new(31.034302, 34.484375),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-686.207, 33136.227),
                half_size: Vec2::new(37.93109, 37.93164),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(291.30475, 33198.816),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(2082.7588, 33294.844),
                half_size: Vec2::new(55.172363, 51.72461),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-2430.0, 33389.77),
                half_size: Vec2::new(50.000122, 80.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(1896.5515, 33363.813),
                half_size: Vec2::new(55.172424, 51.72461),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1990.0, 33389.77),
                half_size: Vec2::new(50.000183, 60.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3030.0, 33569.773),
                half_size: Vec2::new(249.99951, 240.00195),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-968.9656, 33429.33),
                half_size: Vec2::new(100.00003, 96.55078),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-1409.9998, 33449.77),
                half_size: Vec2::new(70.00006, 80.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(-2640.0, 33489.77),
                half_size: Vec2::new(80.0, 100.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(3830.0, 33489.77),
                half_size: Vec2::new(69.99963, 80.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(2468.9658, 33515.535),
                half_size: Vec2::new(82.75842, 100.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(308.69577, 33442.293),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-2980.0002, 33649.773),
                half_size: Vec2::new(160.0, 140.00195),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(308.69577, 33650.99),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
            LevelItem {
                position: Vec2::new(-2049.9998, 33859.773),
                half_size: Vec2::new(70.00006, 70.0),
                thing_type: ThingType::Enemy,
            },
            LevelItem {
                position: Vec2::new(334.7827, 34068.38),
                half_size: Vec2::new(0.0, 0.0),
                thing_type: ThingType::Checkpoint,
            },
        ],
    }
}

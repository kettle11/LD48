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
                position: Vec2::new(-251.72415, 4803.548),
                half_size: Vec2::new(44.827553, 262.06885),
                thing_type: ThingType::Rock,
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
                position: Vec2::new(294.68097, 6992.3613),
                half_size: Vec2::new(3.1915283, 90.42554),
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
        ],
    }
}

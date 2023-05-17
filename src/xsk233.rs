use crate::{from_choice, scalar::Scalar, to_choice, Point};
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq};

#[derive(Eq, Clone, Copy, Debug)]
pub struct Xsk233Point([u64; 16]);

impl Xsk233Point {
    unsafe fn as_xskpoint(&self) -> *const xs233_sys::xsk233_point {
        &*self as *const Xsk233Point as *const xs233_sys::xsk233_point
    }

    unsafe fn as_mut_xskpoint(&mut self) -> *mut xs233_sys::xsk233_point {
        &mut *self as *mut Xsk233Point as *mut xs233_sys::xsk233_point
    }
}

impl Point for Xsk233Point {
    fn add(&mut self, lhs: &Self, rhs: &Self) {
        unsafe {
            xs233_sys::xsk233_add(self.as_mut_xskpoint(), lhs.as_xskpoint(), rhs.as_xskpoint());
        }
    }

    fn sub(&mut self, lhs: &Self, rhs: &Self) {
        unsafe {
            xs233_sys::xsk233_sub(self.as_mut_xskpoint(), lhs.as_xskpoint(), rhs.as_xskpoint());
        }
    }

    fn neg(&mut self, point: &Self) {
        unsafe {
            xs233_sys::xsk233_neg(self.as_mut_xskpoint(), point.as_xskpoint());
        }
    }

    fn double(&mut self, point: &Self) {
        unsafe {
            xs233_sys::xsk233_double(self.as_mut_xskpoint(), point.as_xskpoint());
        }
    }

    fn xdouble(&mut self, point: &Self, n: u32) {
        unsafe {
            xs233_sys::xsk233_xdouble(self.as_mut_xskpoint(), point.as_xskpoint(), n);
        }
    }

    fn mul(&mut self, point: &Self, scalar: &crate::scalar::Scalar) {
        unsafe {
            xs233_sys::xsk233_mul_frob(
                self.as_mut_xskpoint(),
                point.as_xskpoint(),
                scalar.as_void_ptr(),
                scalar.len(),
            );
        }
    }

    fn neutral() -> &'static Self {
        unsafe {
            let sys_ptr: *const xs233_sys::xsk233_point = &xs233_sys::xsk233_neutral;
            let pt_ptr: *const Xsk233Point = sys_ptr.cast();
            &*pt_ptr
        }
    }

    fn generator() -> &'static Self {
        unsafe {
            let sys_ptr: *const xs233_sys::xsk233_point = &xs233_sys::xsk233_generator;
            let pt_ptr: *const Xsk233Point = sys_ptr.cast();
            &*pt_ptr
        }
    }

    fn mulgen(scalar: &Scalar) -> Self {
        let mut out = Self::neutral().clone();

        unsafe {
            xs233_sys::xsk233_mulgen_frob(out.as_mut_xskpoint(), scalar.as_void_ptr(), scalar.len())
        }

        out
    }

    fn decode(&mut self, repr: &[u8; 30]) -> Choice {
        let is_valid =
            unsafe { xs233_sys::xsk233_decode(self.as_mut_xskpoint(), repr.as_ptr().cast()) };

        to_choice(is_valid)
    }

    fn encode(&self, dst: &mut [u8; 30]) {
        unsafe { xs233_sys::xsk233_encode(dst.as_mut_ptr().cast(), self.as_xskpoint()) };
    }

    fn is_neutral(&self) -> Choice {
        let is_neutral = unsafe { xs233_sys::xsk233_is_neutral(self.as_xskpoint()) };
        to_choice(is_neutral)
    }
}

impl Default for Xsk233Point {
    fn default() -> Self {
        Xsk233Point([0u64; 16])
    }
}

impl PartialEq for Xsk233Point {
    fn eq(&self, other: &Self) -> bool {
        unsafe { xs233_sys::xsk233_equals(self.as_xskpoint(), other.as_xskpoint()) == 0xffffffff }
    }
}

impl ConstantTimeEq for Xsk233Point {
    fn ct_eq(&self, other: &Self) -> Choice {
        let is_eq = unsafe { xs233_sys::xsk233_equals(self.as_xskpoint(), other.as_xskpoint()) };
        to_choice(is_eq)
    }
}

impl ConditionallySelectable for Xsk233Point {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut out = Self::default();
        unsafe {
            xs233_sys::xsk233_select(
                out.as_mut_xskpoint(),
                a.as_xskpoint(),
                b.as_xskpoint(),
                from_choice(choice),
            );
        }
        out
    }
}

impl ConditionallyNegatable for Xsk233Point {
    fn conditional_negate(&mut self, choice: Choice) {
        unsafe {
            xs233_sys::xsk233_condneg(
                self.as_mut_xskpoint(),
                self.as_xskpoint(),
                from_choice(choice),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    const DECODE_EXPECT_OK: [&str; 20] = [
        "000000000000000000000000000000000000000000000000000000000000",
        "478f05b8ff56f97ddcdcdda13b42a773be0c1875bff3e555702e2eb1e900",
        "1123b0cbe6de2d68004ae486854ad503c19853bfd3e6c344f572eb66c001",
        "54df1f3c2160b43bff5440ef33fb35bf1740d9188003cfe82f3bede61d00",
        "9e9230101e312faf1e33cd4fd3dd22b01b0a1be8aa1859b9a1efe8ce9e00",
        "1c9cd9aa57f085717b5c977e9e57917eced2f3cf466b264f656f304ad400",
        "0a8c3050729ca55f86095397b4fc26e4b60483f0e89f0a4e4df71030be01",
        "25039e0e09f5c82541c9e1712474c03392786349162853068150d4549a00",
        "71d29dca39024af194526d9902381d9c6c5cdb2f71ab146be5aed028cc01",
        "e97d39f48852edf98b5a1e712599d277743b68cee5680ebc0f1293f60b01",
        "7b7bab22f163a3aed122145c2cb961efc7b8baae8664faf2ea164b752401",
        "ed874112174ec1a2af4cd3e7c179cff6b2cdd92108883dd800f63d0cbb01",
        "64886d03b7e29e43b2a09c2127ac1c6303051dc8dfe60f41d9b61bd66100",
        "c84a0bb20b4b4940f6d7460f08d9c19c3234a92f96d7938b05ef894f1501",
        "f6c9de32061d6e6747cfdaa36987c7df2a5d96814243af976eafaf447501",
        "161d948d488bd26b3d81beca85f8fc96bb1a36e98e78fcd3c3cb90e3c801",
        "be4339d93b7b5dd5e0dfecaa2e309f3525c8dc7c8090ab618a73c7a95001",
        "9023f2233584b06c3a8e958d8569ec3cac91ca4d7c67516ff74e66d4a200",
        "0d37b500c490e66f1e3646b692579295c0b38dc3a1cc261cdf71b696b000",
        "23b6566c74a3de5cef968835f901b7c218108ec6106096ac8c07afadda00",
    ];

    const DECODE_EXPECT_ERR: [&str; 20] = [
        /* w = 1 yields w^2 + w = 0, which is invalid */
        "010000000000000000000000000000000000000000000000000000000000",
        /* trace(b/(w^2 + w + a)) = 1 */
        "5d7ef866575451e3c90a3ed86c19e435e90a0506ec3e56a1d2070c3b6d01",
        "38d0a8b6e69ec6f70872f4a7a235531a696f4b618f58b6c2a1ea40063400",
        "fbc1529dc5347c086493da1a6505115f8e080feebdaa83fc491774d28400",
        "9262fc6b097a43b8a89a53bbf901ad20d41e29aa754881ed71b74bb12e00",
        "1f3096dcb03ffd92d574ee74d1019adaba91aeaadb5bf69da10e83788701",
        "caa9c228ef23944f86557b325488cca9482d274a9bf024b1bd9038d9ff00",
        "12b4c88a9fe72ae9734f17a8df0a017fd29b549e0adb45336897a8c85e00",
        "649e4366355f8a27fff2cd284eca895881fc1fcf798198a572d3be884201",
        "c0bf607a047e032a878a90601c65f139177bcb8d6c7ac25c100b9a383b00",
        /* w maps to points that cannot be halved */
        "5d3f829d1618989458e400e15aeb484c0a4ba3359b7b1f72a6d9ae442d00",
        "6cdd47f7c7d936f068e8e203c343a075d5306a443b1bb5d1c9df6e11d801",
        "eb86e62958e729cfeae85c32e896469a191fa88a019a6f73138d73a73c00",
        "61cf5b9ee3e3a2529f71d8842cd4cc6bc5d7d12c0defae81739409bb2500",
        "7ff9acb794afb0a1f9511af0ca6314a2502ec3c9702cbe00f39f44677e01",
        "b89fc50d28755305cd98f8bb3882178ac8cf231c58e5c3b8217006c9a201",
        "41b08b4fa81064e52e5969eceff573f083f1f9d27ce1013b7a115a6a6100",
        "952722c25a7128a8fa36d600b1b0583fafe333406dda526dd476e3b50600",
        "763d8e4fe9d109435136f92a6d2f3abb9634c57ff4687ef878717a3f2601",
        "4ab33464767e663f3ed4194f606bed91005f7584195c53dd3c4fbc71d201",
    ];

    #[test]
    fn decode() {
        let mut point = Xsk233Point::default();

        for hexstr in DECODE_EXPECT_OK {
            let buf: [u8; 30] = hex::decode(hexstr)
                .expect("error decoding hex string")
                .try_into()
                .expect("parsed bytes of wrong length (!= 30)");
            let is_ok: bool = point.decode(&buf).into();
            assert!(is_ok);
        }

        for hexstr in DECODE_EXPECT_ERR {
            let buf: [u8; 30] = hex::decode(hexstr)
                .expect("error decoding hex string")
                .try_into()
                .expect("parsed bytes of wrong length (!= 30)");
            let is_ok: bool = point.decode(&buf).into();
            assert!(!is_ok);
        }
    }

    #[test]
    fn add() {
        let groups = [
            [
                "ff32132133ff22c3e309ac2a5744ffbe7a745452f9cbc1852591020ef201",
                "285f84c92fe54eba84228362c7ec3b3dc974d3f149d41c53676f0f884c01",
                "b4f7ae8f595620e9993ecec609b220f5a6e55dbcdb72aa193c322a5e5e01",
                "90903e1fb9a714bb872284af141ccb53fe5f36a3b63653e2fdb1f0dd4f00",
                "e6199aa37bb1564c57a0cc3ccd32b921763d724751d98d5917b437022901",
                "a2e81822d3bd47b57ab65618236d6bab4fb9ea267dfd859b409e6292ff00",
            ],
            [
                "8f008e388a7dadcfaf7561c973461c0ab20ca1e112bab7d1849ae019ff00",
                "6fddeb568bd884d259b1752fa4075cda41d7858164c48ac2c631ce8b4d01",
                "deb094684ba2425de6145faaf7ff5465b4ec1b23e564d34b8e7464047c01",
                "8342a39c351a804e2a8035ccc9ed27117b7759ac0cb508f6619f48e04b01",
                "50c1897f1cd8e7882e7ac7216a42a6f197221beaf27c6b299b234d073a01",
                "3b288587d25d167c4f61438aca41484608ced390bc5141e7507d17cbbb00",
            ],
            [
                "1ab150092668e46283529c2377bfead27461e4ae986039b27bdc892ba000",
                "cf6b27d936d46e2c36bf70f3bf67da166b3a52a292864c369153f84a3f01",
                "fb95dcdcc07046fb561c3c11b9f86364c7defbec037c862d472b72e81500",
                "ee418e647aeb98e694145b3a7858f632f14ec3529375be0ddbd8d11dc501",
                "6d924c79d45f4baede7a60f9c99cd6a99dbbee778f900ac906e8e7c83601",
                "8894f5b516b80f2059100086f3e0e1d300a62cbd5879ee5f871d9d2ef700",
            ],
            [
                "bcdd6097170e80beecdf13c9cf01ae15547caf56c5aa8f2fad26864c6100",
                "19e758fc58ed0e700528a1f10f112d4ba8992a1ed60b8a4af76943f22201",
                "fd018b5f0f59dfecab12723018df08705461a3fa2d703cdf7b2696413600",
                "19bdb66fe103ac234b9d3e692df2784bb2ed3b4482485def2bfaf518ac00",
                "3cdd7b97320f9c8f74eb682f4f11a88caa188cdda4e632d3d5748c168700",
                "f5df927b8b1478078e4284b314adfede93412a8a14530a7c115288fd9101",
            ],
            [
                "a25a5d30d88913c6d9c510f60b196ae4b347b76bb8b92fe4bc94d9da1900",
                "3998c97a263bd459c126b498c58f0739605c66bd158d1ca9a45abcd30000",
                "dbcef0d1e6e79e54f52193717f670ca85bc7a0ca7d75e8e74642c2454700",
                "49de384e8fe7764bef3a4489af3199472171962016be84759a6c97e3f801",
                "3e2773a25553e35ca382d1c58dc29fa9d148226b980f89827bc3d3887b00",
                "8cf73f76eb32f4dd704119a2527faffc68721fb8e0e94c7bef5202c96100",
            ],
            [
                "327879816ab3c276a354ca8e33cc4643b1632496c3df17abb9809a46a701",
                "0af4619c45e23ad644aaf8b5d2a90c8675fd512bb90b492ef88df1e59a00",
                "2f2c5f1ad29893c9ffc669821c5465ebfc8cfedba821da069354ea410401",
                "66f95870eb0c28c92d34dcce95e4b0c12f0c9da13200f93bd4f99986d400",
                "1e14eb7e71e0e52d48e584cfd08ba04f4287ad385240388f1e5758db5900",
                "5701404da4c7e44038c147e4a974471a289bc0cbe8a1780e45969bfb4201",
            ],
            [
                "76a96398afbf6be212287eda5671ef8e62116627d9646c0f04081f0f0400",
                "31327483f6edba19761e231d02b53d871c732cd3598f4c8139f716224901",
                "5b85525d41b98e127789fbd15913a3497296d398d558ee28d89df1303401",
                "443d0d33e410953f38ee1e5dfa9666da9bb7bb5a0ed99659d40bd957d600",
                "56de350c2a1c646ae007ba0ff9197bcaf9a664587d9bb2e004489f752201",
                "973d6c27d75380d910dbf25ddf40b0fadc49e1f1a44fa22c4309939d7e01",
            ],
            [
                "d7204c14ef665860bef3aaae0fdd50bb6adab7586b76e490c9959a741801",
                "a88e60081f9c9bb3fa7f559e0e02c60b296fb133700bb9e87f688c929700",
                "5ecb998965809d94752288945a8079348f39e37f998d853312b64a77e001",
                "a662c8f83e0e0834a92145a8fa3a3a4031abc5fc73587d4a182f7fff6a00",
                "e2e68ae840d1bfe0646adae98b854f6362c603ad633d83d0fabc8eca1401",
                "3164384a9b38cd09c4d3cad52d58db04dc6127d5fc730ff845156c715c00",
            ],
            [
                "d600c7021215c93bbb5074e5ce996ae257078fcdcb79ea614f775d6c0600",
                "5a11626fe4d1dc19afd919c409f6c4fa4f60330ebe6db45e0be25da27401",
                "421381d35463de832e33b077f924e2782e787215418129531d4b83bebb01",
                "1d3e836bc15ea3992f7b6997648b3f4f9eadb2887a38c0852eb89250e401",
                "b2bd23c33575b53b4557d6249ceba44c41efe8de213daef4700c352dab01",
                "819cce21e636b7cc6086aa83cc2e0427921dfcc8364c30a7a077021eae00",
            ],
            [
                "2f7dd87f5fd087c349a72d4649f81dbd1f88fc439fc315b0918ac6301501",
                "d3e912c1dfde069bdf37d00ac1932b6fc328c0903d4e15f657407c486500",
                "580bd418c3ee71b271b5024512d4656da8b61ef3e5bd821a9aa133ffe401",
                "00bd8ca49e5be45f9cfa5bf0e22e9e9bb618178e3858b8dbee6659229601",
                "8a3e021f959e3a793e228032fd2330d784ae98b4cc59982675e06c682701",
                "a37b3fadc548f8e54f148a4301e7504a45fd828ef67da67c3851889e2e00",
            ],
        ];

        /*
         * Each group of 6 values encodes points P1 to P6, with:
         *   P3 = P1 + P2
         *   P4 = 2*P1
         *   P5 = 2*P1 + P2 = P4 + P2 = P3 + P1
         *   P6 = 2*P1 + 2*P2 = 2*P3 = P4 + 2*P2 = P5 + P2
         */
        for group in groups {
            let group: Vec<Xsk233Point> = group
                .iter()
                .map(|hexstr| {
                    let mut point = Xsk233Point::default();
                    let buf: [u8; 30] = hex::decode(hexstr)
                        .expect("error decoding hex string")
                        .try_into()
                        .expect("parsed bytes of wrong length (!= 30)");
                    let is_ok: bool = point.decode(&buf).into();
                    assert!(is_ok);
                    point
                })
                .collect();

            let p1 = group[0];
            let p2 = group[1];
            let p3 = group[2];
            let p4 = group[3];
            let p5 = group[4];
            let p6 = group[5];

            let mut q = Xsk233Point::default();
            let mut r = Xsk233Point::default();
            let mut s: Xsk233Point;

            assert!(p1 != p2);

            // xsk233_add(&Q, &P1, &P2);
            // check_b233_eq("add1", &Q, &P3);
            Xsk233Point::add(&mut q, &p1, &p2);
            assert_eq!(q, p3);

            // xsk233_neg(&R, &P1);
            // xsk233_add(&Q, &Q, &R);
            // check_b233_eq("add2", &Q, &P2);
            Xsk233Point::neg(&mut r, &p1);
            s = q;
            Xsk233Point::add(&mut q, &s, &r);
            assert_eq!(q, p2);

            // xsk233_sub(&Q, &P3, &P1);
            // check_b233_eq("add3", &Q, &P2);
            Xsk233Point::sub(&mut q, &p3, &p1);
            assert_eq!(q, p2);

            // xsk233_double(&Q, &P1);
            // check_b233_eq("add4", &Q, &P4);
            Xsk233Point::double(&mut q, &p1);
            assert_eq!(q, p4);

            // xsk233_add(&Q, &P3, &P1);
            // xsk233_add(&R, &P4, &P2);
            // check_b233_eq("add5", &Q, &P5);
            // check_b233_eq("add6", &R, &P5);
            Xsk233Point::add(&mut q, &p3, &p1);
            Xsk233Point::add(&mut r, &p4, &p2);
            assert_eq!(q, p5);
            assert_eq!(r, p5);

            // xsk233_double(&Q, &P3);
            // xsk233_add(&R, &P5, &P2);
            // check_b233_eq("add7", &Q, &P6);
            // check_b233_eq("add8", &R, &P6);
            // check_b233_eq("add9", &Q, &R);
            Xsk233Point::double(&mut q, &p3);
            Xsk233Point::add(&mut r, &p5, &p2);
            assert_eq!(q, p6);
            assert_eq!(r, p6);
            assert_eq!(r, q);

            for j in 0..10 {
                q = p1;
                for _k in 0..j {
                    s = q;
                    Xsk233Point::double(&mut q, &s);
                }

                Xsk233Point::xdouble(&mut r, &p1, j);
                assert_eq!(r, q);
            }
        }
    }

    #[test]
    fn mul() {
        let cases = [
            [
                "65405f2f0a664780b6858a0bc2c458bd3b97eb88df0529c497903eae8601",
                "4e85433e5e2d598f0f644af9cdda327ad9d1eb2f1f00ad73a5d47cf20d5d",
                "87e1fe594929d24b4c5405f5ec03ffedd0d7e30273951ab89a69a06ebb01",
            ],
            [
                "8447778f2982c11888b30bbd5a5418a5a5e309519a32872463674e5e2600",
                "6f92c6bc715b2d69f9ee0b55ccf53cbd86c9e5f3af0a5714992401d700df",
                "1a2fe6248157e11eab2141d8755ec880d528460aa756008990aa928fca01",
            ],
            [
                "ca8c11ffb1e8196df9f55d9b9c00cc05ff609c19a57d1da59e3d842a7100",
                "81b60819e74f7c43f9d55956cb9af346c791ab634b9e91707c426cd58d8d",
                "7dd17870ceeb3f3f221ef20707b3e2ac5fa6e16139e92ef85b514940e700",
            ],
            [
                "88c4737cc61c737d9a3177f4f63804558685b8518a9b4899e822bd3acf00",
                "92b9ee4dd598039761281aac140fd8802b600b0ea98d82c8567d2ee8bfa0",
                "b3e29bd8a8b5b1fb76bcdea37d3fd78ab409d0d982532c7b9f8079a9ba01",
            ],
            [
                "cf83e5135b57ded363e44c88cb1d5925a9f18332308d636938357191f500",
                "4c9c704abe0ee618f255596decaf323f9c3c7d6b35d1f0ef9f7f3a963c69",
                "94a8a5040bb6ae2d7bb12a44e677bb6fa843f46c4473a4a8a399391d3a00",
            ],
            [
                "08b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                "b8dbd51d8cb04149bcd55196901d77767fca295c87a80a8bb0dfd3abf401",
            ],
            [
                "08b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
                "deab73f1d51afb6ed4bc15b95b9d06000000000000000000000000008000",
                "09b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
            ],
            [
                "08b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
                "dfab73f1d51afb6ed4bc15b95b9d06000000000000000000000000008000",
                "000000000000000000000000000000000000000000000000000000000000",
            ],
            [
                "08b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
                "e0ab73f1d51afb6ed4bc15b95b9d06000000000000000000000000008000",
                "08b0df6178ed276829b40449331bc7106dca3475ed6e296305a0017da500",
            ],
            [
                "000000000000000000000000000000000000000000000000000000000000",
                "ac47ee67e70933727c304bae789bd08aff54ff56d5240f4996ef21976a58",
                "000000000000000000000000000000000000000000000000000000000000",
            ],
        ];

        for case in cases {
            let mut start_point = Xsk233Point::default();
            let mut exp_dest_point = Xsk233Point::default();

            let buf: [u8; 30] = hex::decode(case[0])
                .expect("error decoding hex string for start point")
                .try_into()
                .expect("parsed bytes of wrong length (!= 30)");
            let is_ok: bool = start_point.decode(&buf).into();
            assert!(is_ok);

            let scalar_buf: [u8; 30] = hex::decode(case[1])
                .expect("error decoding hex string for scalar")
                .try_into()
                .expect("parsed bytes of wrong length (!= 30)");
            let scalar = Scalar::new(scalar_buf);

            let buf: [u8; 30] = hex::decode(case[2])
                .expect("error decoding hex string for expected destination point")
                .try_into()
                .expect("parsed bytes of wrong length (!= 30)");
            let is_ok: bool = exp_dest_point.decode(&buf).into();
            assert!(is_ok);

            let mut dest_point = Xsk233Point::default();
            Xsk233Point::mul(&mut dest_point, &start_point, &scalar);
            assert_eq!(dest_point, exp_dest_point);
        }
    }

    #[test]
    fn encode() {
        for (i, hexstr) in DECODE_EXPECT_OK.iter().enumerate() {
            let mut point = Xsk233Point::default();

            let buf_slice = hex::decode(hexstr).expect("couldn't decode hex string");
            let mut buf1 = buf_slice.try_into().expect("wrong length");
            let is_ok: bool = point.decode(&buf1).into();
            assert!(is_ok);

            let is_neutral: bool = point.is_neutral().into();
            if i == 0 {
                assert!(is_neutral);
            } else {
                assert!(!is_neutral);
            }

            let mut buf2 = [0u8; 30];
            point.encode(&mut buf2);

            assert_eq!(buf1, buf2);

            let last_byte = buf1[29];
            for mask in 1..128u8 {
                buf1[29] = last_byte | (mask << 1);
                let decode_ok: bool = point.decode(&buf1).into();
                let is_neutral: bool = point.is_neutral().into();
                assert!(!decode_ok);
                assert!(is_neutral);
            }
        }
    }

    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn mulgen() {
        let mut rng = ChaCha8Rng::from_seed([42u8; 32]);

        for _ in 0..20 {
            let mut buf = [0u8; 30];
            rng.fill(&mut buf);

            let scalar = crate::scalar::Scalar::new(buf);
            let mulgen_point = Xsk233Point::mulgen(&scalar);
            let mut mul_point = Xsk233Point::default();
            mul_point.mul(Xsk233Point::generator(), &scalar);

            assert_eq!(mul_point, mulgen_point);

            // pornin's c library also tests whether this works for shorter scalars, but we
            // currently don't have shorter scalars, so we can't do that. maybe we could put that
            // on the roadmap though.
        }
    }
}

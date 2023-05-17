// this file is mostly the same as xsk233.rs, except that
// - we call the xsb233_* functions instead of xsk233_* functions
// - we don't use the frobenius endomorphism for multiplication

use crate::{from_choice, scalar::Scalar, to_choice, Point};
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq};

#[derive(Eq, Clone, Copy, Debug)]
pub struct Xsb233Point([u64; 16]);

impl Xsb233Point {
    unsafe fn as_xsbpoint(&self) -> *const xs233_sys::xsb233_point {
        &*self as *const Xsb233Point as *const xs233_sys::xsb233_point
    }

    unsafe fn as_mut_xsbpoint(&mut self) -> *mut xs233_sys::xsb233_point {
        &mut *self as *mut Xsb233Point as *mut xs233_sys::xsb233_point
    }
}

impl Point for Xsb233Point {
    fn add(&mut self, lhs: &Self, rhs: &Self) {
        unsafe {
            xs233_sys::xsb233_add(self.as_mut_xsbpoint(), lhs.as_xsbpoint(), rhs.as_xsbpoint());
        }
    }

    fn sub(&mut self, lhs: &Self, rhs: &Self) {
        unsafe {
            xs233_sys::xsb233_sub(self.as_mut_xsbpoint(), lhs.as_xsbpoint(), rhs.as_xsbpoint());
        }
    }

    fn neg(&mut self, point: &Self) {
        unsafe {
            xs233_sys::xsb233_neg(self.as_mut_xsbpoint(), point.as_xsbpoint());
        }
    }

    fn double(&mut self, point: &Self) {
        unsafe {
            xs233_sys::xsb233_double(self.as_mut_xsbpoint(), point.as_xsbpoint());
        }
    }

    fn xdouble(&mut self, point: &Self, n: u32) {
        unsafe {
            xs233_sys::xsb233_xdouble(self.as_mut_xsbpoint(), point.as_xsbpoint(), n);
        }
    }

    fn mul(&mut self, point: &Self, scalar: &Scalar) {
        unsafe {
            xs233_sys::xsb233_mul_ladder(
                self.as_mut_xsbpoint(),
                point.as_xsbpoint(),
                scalar.as_void_ptr(),
                scalar.len(),
            );
        }
    }

    fn neutral() -> &'static Self {
        unsafe {
            let sys_ptr: *const xs233_sys::xsb233_point = &xs233_sys::xsb233_neutral;
            let pt_ptr: *const Xsb233Point = sys_ptr.cast();
            &*pt_ptr
        }
    }

    fn generator() -> &'static Self {
        unsafe {
            let sys_ptr: *const xs233_sys::xsb233_point = &xs233_sys::xsb233_generator;
            let pt_ptr: *const Xsb233Point = sys_ptr.cast();
            &*pt_ptr
        }
    }

    fn mulgen(scalar: &Scalar) -> Self {
        let mut out = Self::neutral().clone();

        unsafe {
            xs233_sys::xsb233_mulgen(out.as_mut_xsbpoint(), scalar.as_void_ptr(), scalar.len())
        }

        out
    }

    fn decode(&mut self, repr: &[u8; 30]) -> Choice {
        let is_valid =
            unsafe { xs233_sys::xsb233_decode(self.as_mut_xsbpoint(), repr.as_ptr().cast()) };

        to_choice(is_valid)
    }

    fn encode(&self, dst: &mut [u8; 30]) {
        unsafe { xs233_sys::xsb233_encode(dst.as_mut_ptr().cast(), self.as_xsbpoint()) };
    }

    fn is_neutral(&self) -> Choice {
        let is_neutral = unsafe { xs233_sys::xsb233_is_neutral(self.as_xsbpoint()) };
        to_choice(is_neutral)
    }
}

impl Default for Xsb233Point {
    fn default() -> Self {
        Xsb233Point([0u64; 16])
    }
}

impl PartialEq for Xsb233Point {
    fn eq(&self, other: &Self) -> bool {
        unsafe { xs233_sys::xsb233_equals(self.as_xsbpoint(), other.as_xsbpoint()) == 0xffffffff }
    }
}

impl ConstantTimeEq for Xsb233Point {
    fn ct_eq(&self, other: &Self) -> Choice {
        let is_eq = unsafe { xs233_sys::xsb233_equals(self.as_xsbpoint(), other.as_xsbpoint()) };
        to_choice(is_eq)
    }
}

impl ConditionallySelectable for Xsb233Point {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut out = Self::default();
        unsafe {
            xs233_sys::xsb233_select(
                out.as_mut_xsbpoint(),
                a.as_xsbpoint(),
                b.as_xsbpoint(),
                from_choice(choice),
            );
        }
        out
    }
}

impl ConditionallyNegatable for Xsb233Point {
    fn conditional_negate(&mut self, choice: Choice) {
        unsafe {
            xs233_sys::xsb233_condneg(
                self.as_mut_xsbpoint(),
                self.as_xsbpoint(),
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
        "8940bd43927c8b2947b945b3186132156182142eae619a59d2e4485b7500",
        "8fccbab7e7cd1f22ed1e61fe62bbb3482ac9d8d9495ab940d50900673f01",
        "05d37ff3b74b7ce54600866bece0602ff3387f85e4719705a68be2fa5500",
        "d8c9bacab5039f5c9cf60f54852161944c0a03bd4573a1768d2ebe9d8e01",
        "7fb5e0348570597d57c1d2fca09c7f65b31983ab93022740027aa0efba01",
        "ed003b7ab53d132fc0ef1fa36e6535b47290184a3393911009982a819c01",
        "fc78318fc05ebaf2b047f3bc65ffe39b580615a9068496c1977f0bc9cc01",
        "30058d4b2ccd42614fb59b9ef03cf6cd355c140f83fdfe55fe8fb2666501",
        "a44b02f35d234068a67d8498bdcf81098baf54cb49268ce5cb9e2eef5901",
        "0013dffa8bfe74dfb0f54e79ad9b7ce0ce32ade12b58595c6587b67e4401",
        "55b6ad37c6fd5df261e6ac9b2d74536addfb66646bd8e8296a7f6324ce00",
        "47b3b75b1a995e96a672edb8c716839bb589212965e2deb03c7a6ccf6001",
        "82fdb5a8465dd6c59b10dde5432c353edc41577b124f8ef7d0a8f509c500",
        "558f4447507fb52cc08d4fb34f4ad4a2479a3d2246038fd9f0942c9fe600",
        "a24dd168d1d3f75f81a8b8afc31947bdaf166839d8195f9937ff8b581400",
        "bfea156043b9be7dc7c3cd410a0c38f0b2e9a1fd0f414a4fa9ea2cab1001",
        "a97e14af5cf95c8624ba422c57ecec7d898a1e6cc2a10a3e667472cf9e01",
        "5151b9560be3f48042f4adde5d603e0ece23d4d9d7072215b94a78b6ad00",
        "50c08e4f6ec9f4feb32152c19809879983422eb54200eb50792600d73901",
    ];

    const DECODE_EXPECT_ERR: [&str; 20] = [
        "801fb075ad4b6708d7ca674fd29f45118a1e98f34672e6f416db57ba5a01",
        "3c098046550a679c35d1aedb20a4a5d52f4342ba1b79f0a7d2c27a77b600",
        "6771ec2e4a8409b328f591addc14a740502dc293888d761d2a2093b9d501",
        "b18bcf83211455363978c178dcf28c3e8d643d816124067a0c0b0438fd00",
        "7c17bbf7a92f393988465f1d7d1724c8b8510352b077efc82d00115c7400",
        "a260b4bafc91b68f9596cfa114a0813fa57397df479d6a751a204e2f8500",
        "88d7b4a8edd9640f151d9cbd95f77974ae46c79067be72cf9bd421528800",
        "2b19b2b57c4cee44c53c02bb7f2c542000e6f97266a47acd450b3223ff00",
        "97e0a30a19fc45a454d0d4fb703376c80d91751d50b16809e26e2fef3601",
        "646bc8733e38db5f6397f78698f3dc9931eec60344eec49be18fe96d6f01",
        "c01a93fb2fe1e0b9f2e61062a7ec0d5339071c2ce90aebeb10afd79c1600",
        "df7fe9d661debfb6b0de91b204c0dace85eea241d198594b509bb4e49000",
        "3b67cfc6f70edd422c3cc8855e41193563f94074558f5721dcc030b47e01",
        "5d95d619e23e2ffdbb6a780391ea8305e78755b8e80ec94ef71ed3a55001",
        "acdbb3d7959c65c0229a28908222dbc7332b3f643c036f7f2d76bc37bd01",
        "c16c24fdda4895754181cb844438d3a3672b799eeb6d603318c12ca83001",
        "9ef7db1233d0c69ae03b81d3637409487b80d5de2c797b3e9c4ec9bac501",
        "d9d2e06213b9a2873401f030f1800b0af22c41bfa7a5af04a9f5930bfc01",
        "ce36a5a61c4e8559700cbf3f3cf730a4dd663c14badf9b317acf1555af00",
        "1ad612531e5bb81d9c4dccc6725673a28fdf9ea787d5ec3f056c36f65900",
    ];

    #[test]
    fn decode() {
        let mut point = Xsb233Point::default();

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
                "02144880b80e145111038b896e3353909604af0b093c53dcb97957c85d01",
                "76d4b7e2c85b59dad633d0b193e4a081640bbcaec951ce756f19bce62c00",
                "9ab06eca75f4afdb3fd838f199ea2efccbedf23632dd2642e64d44c4dc00",
                "eb929217cae07eeabb3a2f73f0529b815c36a0b8dd622e7e112c39728a01",
                "8776ebf4b14066972fdbf06418e2e7e1ef30c7958c7508ee5f12dffd3401",
                "a27faa8d0117ca49b9effa1f4104a42bcb7848014f11c02ff4f4db9a3801",
            ],
            [
                "4d58931f05fa526cd4f07de21c71e910cdfd8419a1c3ca455b788c68b201",
                "ca028772e5db4cd219459467d5ca68d27e631e6aa68addffc9834e4ba301",
                "c6f6f201503566437c881d3561f9f8e0029d15169a90852dcb96ef257b01",
                "0c63dc67e9e9087f3b1e857dd769615633eda9849a05409a12347c6c9e00",
                "ac89dd4af3a0208edbcc1e78ebd0eeb0ccd28d56329806f2209ed287a501",
                "8de7a7a4ecc484f8536983c2bb6b6dc483ef06adc103f7116360c07c4301",
            ],
            [
                "2cebbc366c6c1f62ee690b370e970c1a718cf016c9d3b03c58485eca5f01",
                "a61388f4a96f1fbe5f19a43ae9a06794fb4120abd03d3116ad434cb7b400",
                "3d033c423aec126e2ccf21eeff0840c413a6054e57a84c5ef4a87811e100",
                "df66d1ff0019d8a0e3cb613caec460c82ef8059039c9a64216a2f9b0e501",
                "48f09a5cefe7b7d56cbf3ed5d19d17d84888b9fcc17d16b8ad3d189c3401",
                "216ae8510928fc21c4bcc9c152fae5be65f2423c202af94482d45e508c00",
            ],
            [
                "b1b346bee688b3dcb279315e0f64ad7b71abb413ff26b9cf54accd361300",
                "40af3372e5e28d72bbee0208b9e088407bf35764d218d5c906ff0df98b01",
                "382bc13161d0e039aad71378c0d35f59ee67e969a18c0c1ceba5fc9f2900",
                "8001a637c126818815a11d801ef488777f2d2936a2a1bece94c52abecc00",
                "a569aa7b2165a7144aeb49000c0f1f74e6cbd020d24fe995bad98cc7ad00",
                "a80950085f1b28aaada3753a99f97cd5d5180ffe5d4f7d456efccc9f9600",
            ],
            [
                "11e4433fc1402ed10029d41212289ea399d41db9d40a388437ee8a5ebe01",
                "436dee22637a2870237e450541a21828249948585adbd2259a596a22fa01",
                "cd6425529751c0b87b95181b2698460693295760d99713c1abd5cb6d8401",
                "75314bc24edd1afb65bbf2eac38fe8ef1f144fa3930b48de2ea895cb3001",
                "123ce9b917e18a032cc3721f3b1850a16acd4d71bc0f7e6987797645fa01",
                "2b9a20351e0ce435886d22690ddeadf1d2303f38d8737e4471dbc7681c00",
            ],
            [
                "e6396f9bcff1106c56e83bd57c68e0c3ce272bcf5f9a6c14e7ffb6e7ca01",
                "c10a69de3fa6394dbbab9128afb2a23fd3cfdbf8f8cf6c456e4710c97300",
                "07ceb73e995462cf4b10979e307d6b8b0d5051a783c6f3ecf4513d7e2e00",
                "ffa05d8cbf717b21e5da33a87c2da01df0f0147861624a93a3f8e62aa601",
                "04fc2cb7f8ee34db20c977ca5e5582581fe515cf4eb8a7ccd7401838a401",
                "f4718c6b783852485210420262b5974e3e635a13a805222d61a072a5d501",
            ],
            [
                "7dd5eed5adae63360d282e011ec3596c625a5f91b91dbd85209b550a1000",
                "c91d0d0367efe902b1af07496984acb6c9daa3f973714fd9a5c68741c400",
                "4d4c928d5963c2013f7484eecad069bcc78fb4216009a3b0cc556e59d601",
                "5d902227902048774c922bbaeaa456ec52f4983f56d1d641f96b378ec801",
                "9e4aeed2b6bd766dc5873de2a6c1d93f5a262113a51f0582b460c107af00",
                "51597210f9392667c38a1e4942a7b0b83782288a245336f1606c51924700",
            ],
            [
                "3b47769eb490fc588f4251ee38bc8c70423392d6c2381c8803924aa89e01",
                "c33240af7431a1d9cf1bc45d4737f35a9a2b8b8473d6db8459de7b421000",
                "674b3829a9c85ca207e710d762460bef6ddfc92bc47cfde4ddf6206fd901",
                "a37ac566fef860a5a55e1db8599a95867cdf3161726e2f3828d758b0b901",
                "99e73c60a1a9501fcff7b63017cbe8303087a9c12491bfedffa583e48e00",
                "b1c1207cc51c742a7c55afcedcb10fe78951573361e7115e5d1e55522b00",
            ],
            [
                "2a852fede5ff207503879030383a20829439f538a7844f61b0e49cee7000",
                "537dea25a66b6995f631b9688117db9470ff4e1032358f9a9020a4f63500",
                "3fa567c96910c09678f9b7609e0aec6f1a40d080d9d2166ad9ef46881701",
                "d7a17d8bd7f84ebde0ddef19beb1165142c4519114e454f1d8017322d200",
                "c551dc502d0915c04459999f5e37ec4a915284371a61396435086ab49201",
                "6444ccb3dc00bdf2103d46e663778b5514ff1d7a848c2df9f3a96cdfe900",
            ],
            [
                "77ba1b3758f1ee9efd99a6059224782ba1b92fb00beb3db53d984635d901",
                "fd5029736e2062ad024ccfb5e0e63c6489b537f3587c0c4d8bd47a5a2d00",
                "8036e6f8f53eedf796fa59fae533b743f0c11b955eb8e68c71105bc76600",
                "633c9c5da54168013f4a937aa1d0dc346c579def1a7e54484efa45192301",
                "cfa8569d09b38a25129c61e552e60093d078065901ec007bd422649fdf01",
                "d8a82435a2806938e5f69adbbdd930efc2603e898182d4c1f289127bc300",
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
            let group: Vec<Xsb233Point> = group
                .iter()
                .map(|hexstr| {
                    let mut point = Xsb233Point::default();
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

            let mut q = Xsb233Point::default();
            let mut r = Xsb233Point::default();
            let mut s: Xsb233Point;

            assert!(p1 != p2);

            // xsb233_add(&Q, &P1, &P2);
            // check_b233_eq("add1", &Q, &P3);
            Xsb233Point::add(&mut q, &p1, &p2);
            assert_eq!(q, p3);

            // xsb233_neg(&R, &P1);
            // xsb233_add(&Q, &Q, &R);
            // check_b233_eq("add2", &Q, &P2);
            Xsb233Point::neg(&mut r, &p1);
            s = q;
            Xsb233Point::add(&mut q, &s, &r);
            assert_eq!(q, p2);

            // xsb233_sub(&Q, &P3, &P1);
            // check_b233_eq("add3", &Q, &P2);
            Xsb233Point::sub(&mut q, &p3, &p1);
            assert_eq!(q, p2);

            // xsb233_double(&Q, &P1);
            // check_b233_eq("add4", &Q, &P4);
            Xsb233Point::double(&mut q, &p1);
            assert_eq!(q, p4);

            // xsb233_add(&Q, &P3, &P1);
            // xsb233_add(&R, &P4, &P2);
            // check_b233_eq("add5", &Q, &P5);
            // check_b233_eq("add6", &R, &P5);
            Xsb233Point::add(&mut q, &p3, &p1);
            Xsb233Point::add(&mut r, &p4, &p2);
            assert_eq!(q, p5);
            assert_eq!(r, p5);

            // xsb233_double(&Q, &P3);
            // xsb233_add(&R, &P5, &P2);
            // check_b233_eq("add7", &Q, &P6);
            // check_b233_eq("add8", &R, &P6);
            // check_b233_eq("add9", &Q, &R);
            Xsb233Point::double(&mut q, &p3);
            Xsb233Point::add(&mut r, &p5, &p2);
            assert_eq!(q, p6);
            assert_eq!(r, p6);
            assert_eq!(r, q);

            for j in 0..10 {
                q = p1;
                for _k in 0..j {
                    s = q;
                    Xsb233Point::double(&mut q, &s);
                }

                Xsb233Point::xdouble(&mut r, &p1, j);
                assert_eq!(r, q);
            }
        }
    }

    #[test]
    fn mul() {
        let cases = [
            [
                "106119152d4352ee7b4f5477bf024d5b03652787ce6d65a1873378cf3c00",
                "e9b6249bbbe83e5acdc45173c92421b785fe8d108c48468f0731fd43a834",
                "dc7990d625cde144b98d41b5bc7745c1f46e07592ce0d7c684b6e8d86500",
            ],
            [
                "462375f85cbea6f0c9b6a966e6c6605e0b52c1020263b64deafaf5675801",
                "240944c3a4fb54f9cadd6a182e19715a62e62161632f0ca913540fb58f69",
                "e4a62e179f93d992669c8a763f368b4cd149bac0210ae91c92d66d361001",
            ],
            [
                "85f70654a469dcff1802afcf7f65178a9a8d4b9093a076c10fb6a1eb6f01",
                "5c42bdc259deb257843bcae0eaa0d9b96bb43688f2319776f5876a651b03",
                "81ca9c26fa51bdfc603e7d66397e91f1be95ac5467a45eb747b1c21e4700",
            ],
            [
                "f514c5c2ff9b864f1ed7b5cd1fff963356656e06f68c29f076a73a661a01",
                "c6f34cd3e3c369e86bd2f3792a3dfab2a51902c36eab85ca0af1779b88f9",
                "107772b42be2ff485d96849e7a0065bb4c6a57759c00d0ec6443dfd53501",
            ],
            [
                "ef0469fb216d019185491789b5f2f44b69fd3934473e3cfda7f59d16a501",
                "78def7a7ac41131b0d843339b8af2d0107067d350c75f77fc59709ca6b92",
                "90e1bb7a8f748c09e95224aef5a110adf8c6c058e89167f9d19c68edc901",
            ],
            [
                "baff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
                "507e493c422fededb0b0128f1d6cf6f295cadfca76c91702eb0260cf4701",
            ],
            [
                "baff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
                "d6e0cf03261d0322698a2fe774e913000000000000000000000000000001",
                "bbff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
            ],
            [
                "baff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
                "d7e0cf03261d0322698a2fe774e913000000000000000000000000000001",
                "000000000000000000000000000000000000000000000000000000000000",
            ],
            [
                "baff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
                "d8e0cf03261d0322698a2fe774e913000000000000000000000000000001",
                "baff096082f63d76bf21418783b1107529290a640569abccd9178e894900",
            ],
            [
                "000000000000000000000000000000000000000000000000000000000000",
                "f26efbaa86ce71af4affd3d368abb6ea372d3b426d617313b1916fb48cba",
                "000000000000000000000000000000000000000000000000000000000000",
            ],
        ];

        for case in cases {
            let mut start_point = Xsb233Point::default();
            let mut exp_dest_point = Xsb233Point::default();

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

            let mut dest_point = Xsb233Point::default();
            Xsb233Point::mul(&mut dest_point, &start_point, &scalar);
            assert_eq!(dest_point, exp_dest_point);
        }
    }

    #[test]
    fn encode() {
        for (i, hexstr) in DECODE_EXPECT_OK.iter().enumerate() {
            let mut point = Xsb233Point::default();

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
            let mulgen_point = Xsb233Point::mulgen(&scalar);
            let mut mul_point = Xsb233Point::default();
            mul_point.mul(Xsb233Point::generator(), &scalar);

            assert_eq!(mul_point, mulgen_point);

            // pornin's c library also tests whether this works for shorter scalars, but we
            // currently don't have shorter scalars, so we can't do that. maybe we could put that
            // on the roadmap though.
        }
    }
}

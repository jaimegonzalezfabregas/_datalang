#[cfg(test)]

mod tests {
    use crate::engine::Engine;

    #[test]
    fn query_full_table_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0, 1)\n",
            engine.input("rel(0,1) rel(_,_)?".into(), false)
        );
    }

    #[test]
    fn query_full_table_2() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0     , 1)\n(\"hola\", 1)\n",
            engine.input("rel(0,1) rel(\"hola\",1) rel(_,_)?".into(), false)
        );
    }

    #[test]
    fn query_filter_table_2() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(\"filtro\", 1)\n",
            engine.input(
                "rel(\"clave\",1) rel(\"filtro\",1) rel(\"filtro\",_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0)\n(1)\n(2)\n(3)\n",
            engine.input(
                "rel(0,1) rel(2,3) test(a) :- rel(a,_) || rel(_,a) test(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_2() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(3)\n(4)\n",
            engine.input("rel(4,4) rel(0,1) rel(2,3) rel(2,2) !rel(2,2) rel(3,3) test(a) :- rel(a,a) test(_)?".into(), false)
        );
    }

    #[test]
    fn view_resolving_verbose_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel(0) relSuc(suc) :- rel(a) && a = suc-1 relSuc(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_resolving_verbose_2() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel(0) relSuc(suc) :- rel(a) && a+1 = suc relSuc(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_resolving_verbose_1_reverse() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel(0) relSuc(suc) :- a = suc-1 && rel(a) relSuc(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_resolving_verbose_2_reverse() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel(0) relSuc(suc) :- a+1 = suc && rel(a) relSuc(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_resolving_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input("rel(0) relSuc(suc) :- rel(suc-1) relSuc(_)?".into(), false)
        );
    }

    #[test]
    fn view_resolving_2() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input("rel(0) relSuc(a+1) :- rel(a) relSuc(_)?".into(), false)
        );
    }

    #[test]
    fn view_resolving_3() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel1(0) rel1(1) rel2(1) rel2(2) test(a) :- rel1(a) && rel2(a) test(_)?".into(),
                false
            )
        );
    }

    #[test]
    fn view_resolving_4() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel1(0) rel1(1) rel2(1) rel2(2) test(a) :- rel1(b) && rel2(c) && b=c && a=b test(_)?"
                    .into(),
                false
            )
        );
    }
    #[test]
    fn view_resolving_5() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input(
                "rel1(0) rel1(1) rel2(1) rel2(2) test(a) :- b=c && a=b && rel1(b) && rel2(c) test(_)?"
                    .into(),
                false
            )
        );
    }

    #[test]
    fn equation_resolving_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(1)\n",
            engine.input("test(x) :- 0 = x - 1 test(_)?".into(), false)
        );
    }

    #[test]
    fn view_resolving_and_projection_1() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(2)\n\nEmpty Result\n\n(2)\n(3)\n",
            engine.input("rel(1) rel(2) rel(3) rel(4) inner(x) :- rel(x) && rel(x+1) && rel(x-1) inner(2)? inner(4)? inner(_)?".into(), false)
        );
    }

    #[test]
    fn arrays_1() {
        let mut engine = Engine::new();
        assert_eq!(
           "\n(3, [2,1])\n(6, [5,2])\n",
            engine.input("rel([1,2,3]) rel([6,5,2]) rel([3,2,1]) test(a,b) :- rel([a,...b]) && a > 2 test(_,_)?".into(), false)
        );
    }

    #[test]
    fn recursion() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0)\n(1)\n(2)\n(3)\n(4)\n(5)\n",
            engine.input(
                "test(a+1) :- test(a) && a < 5 test(0) test(_)?".into(),
                false
            )
        );
    }

    // #[test]
    // fn array_recursion() {
    //     let mut engine = Engine::new();
    //     assert_eq!(
    //         "\n([0])\n([1])\n([2])\n([3])\n([4])\n([5])\n",
    //         engine.input(
    //             "test([a+1]) :- test([a]) && a < 5 test([0]) test(_)?".into(),
    //             false
    //         )
    //     );
    // }

    // #[test] // 
    // fn arrays_3() {
    //     let mut engine = Engine::new();
    //     assert_eq!(
    //         "",
    //         engine.input(
    //             "reverse([a,...b],ret) :- reverse(b, rb) && ret = rb + a reverse([a],[a]) :- 1 = 1 reverse([1,2],_)?".into(),
    //             false
    //         )
    //     );
    // }

    #[test]
    fn double_constraint_and() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0)\n(2)\n",
            engine.input(
                "a(0) a(1) a(2) a(3) b(0) b(2) b(4) b(6) ayb(x) :- a(x) && b(x) ayb(_)?".into(),
                false
            )
        );
    }

        #[test]
    fn double_constraint_or() {
        let mut engine = Engine::new();
        assert_eq!(
            "\n(0)\n(1)\n(2)\n(3)\n(4)\n(6)\n",
            engine.input(
                "a(0) a(1) a(2) a(3) b(0) b(2) b(4) b(6) ayb(x) :- a(x) || b(x) ayb(_)?".into(),
                false
            )
        );
    }
}

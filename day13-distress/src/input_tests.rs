#[cfg(test)]
mod tests {
    use day13_distress::*;

    // From the actual input
    #[test]
    fn input_1() {
        let input = "[[],[[3,[0,3,0,7,9],3],[0,1]],[[],4],[[[8,5]]],[9,5,5,1]]\r\n[[[[3,1,9,3],[1,1,1,9],1,[4,9,3,2],[9,1,8,7]],[1],[[4,2,10,2,5],[]],[[9,7,1],5,2,2,10],2],[],[[10,10,[],0]]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_2() {
        let input = "[[[5],7,0]]\r\n[[3,8,5],[9,[[4,10,0,4],[2,8,8,8,1],[2,6],[9,5,1,0,5]],10,6,5],[]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_3() {
        let input = "[[3,7,9,[[0,7,0]]],[[[7],[2,5,9,1],[9],9],2,[6,8,[9,2,2,9,3],[6,8]]],[[[2,1,7,10,5]],9,8,[9,1,5,[]]],[4,4,10,[2,[0,1,1,1],[10,3],10,[4,2,1,5,5]]],[[],9,7,9]]\r\n[[1,4,[],[9,0],[0,[7,4],8,1,[10,1,1,2]]],[[[10,6,6],[]]]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_4() {
        let input = "[[[[3,9,6,7,8],[],10,10]],[[10,[],[]],[[1]],4,[]],[5,8,7],[[2,6,0],[3,[3,7,10,8,6]],6],[9,0]]\r\n[[0],[10,[],5,2,[[4],[0,6],8]],[3],[9,1,3]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_5() {
        let input = "[[5],[[[6,2,5],10,[6,10,3,10,8],1],1,[[7]]],[7,[7,1,3],1],[5]]\r\n[[4],[[4],[[2,6,1],6,8],[1],[[6],[6,3]]],[[[6]],[[3],[10],[0],[]],8,[8,9,4],[[8,4,7],8,[10,0,8,3,1]]],[[[6,0,10,9,1],[],0],10,[2,[5,7,2],[5,8,3],[8,10,4],[8,7]],9],[[0,[9,10],[5,9,4,9,7]],5,4]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_6() { // This is about list depth
        let input = "[[],[],[[[8,10,8,1,8],7,[],[10,0,0,4]],8,9,[3],[1,[5,0],[3,8,3,0],5]],[7,[7,2,[]],4,8,5],[]]\r\n[[7,[1,5,[6,7,8,4],[],[]],7],[[],[[],10,5]],[2,0,[10,[8,6,10],10],[0,[],0,[10],[9,9]]]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_7() {
        let input = "[[2,[[10],1,1],6,2],[7,7,[],[[4,4],[4,7,7,6],[1,3,7]],7],[3]]\r\n[[],[0],[1,[],1,3,9],[0,10,5,4],[7,4,1,[[3,4],9,[7,5,8],[1,10,5,0]]]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_8() {
        let input = "[[5,4,[[8,7,9,7,7],[],8],[[7,6,3],[10,3],[6,2,2,4]]]]\r\n[[[[0,8],2,[6,8,2,1,7],[0,4,7,9]],3],[8,3],[],[3,[[0,1,7,2,9],[9]]]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_9() {
        let input = "[[9],[[[2,8,1,4],[3,2,2,5],[2],6]]]\r\n[[2,10,7],[]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_10() {
        let input = "[[6,[3,1,2,[],[8,6,7,7]],2,[4,3,[3,4,8,3,5]],8]]\r\n[[[[4,5,5,7,2],[0,1],7,3,[]],4],[[[6],1,[5,3,5,2,7],[6,3,2],3]],[7,0,10,2,[2,8]]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_11() {
        let input = "[[],[[[3,1,4],4,8]]]\r\n[[2],[[0,0],[]],[8],[4,9,5,[6]]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_12() {
        let input = "[[],[[[6],[6,5,2],[6]]],[6,4,7,0]]\r\n[[[[0,2,3],3,[8,2,1],6,2],[[],[],[4,5,4],4]],[[9,[0],8,5]],[[8,[2,0],[0,4,8],10,1],[10]]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_13() {
        let input = "[[[6]],[]]\r\n[[8,[],7,10],[],[10,[[0,8,5,8,10]],6,1,[[9],[5,2,7,1],[3,1]]],[[[6,5],[]]],[0,5]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_14() {
        let input = "[[[[6,8,10,8],[]]]]\r\n[[[9,[2,2,0]],[3]],[[5]],[]]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn input_15() { // 43-44
        let input = "[[[2,[8,1,1],[9],3,[1,3,6,1,6]],[]],[2,[7,[8,8,3],[],[6],[1,4]],3],[[5],[[],5,2,6],2,10],[[[]]],[5,[[10,7,3,3],[5,4,2,1,8],8,4],5,[2,6]]]\r\n[[[],[[1,8],3],4,2,[[9,7,3,0],7,[9,6,7,7],[],[3,7]]],[[[4,5]]]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_16() { // 79-80
        let input = "[[2,8,[],0],[],[3,6],[4]]\r\n[[],[[[9,0,5,7,7],0,4,4]],[[],[[4,0,0,6,10],2,[3,0]]],[]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn input_17() { // 91-92
        let input = "[[1,[],[]]]\r\n[[],[],[5,[[4,5,1,5,5],5]],[[],[]],[]]";

        assert_eq!(is_ordered(input), false);
    }
}
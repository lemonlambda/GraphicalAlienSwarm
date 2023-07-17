#[test]
pub fn tile_item_test() {
    use graphical_alien_swarm_proc_macros::tileitem;

    struct VariantInternal {}
    struct AutotileInternal {}
    
    #[tileitem(VariantInternal, AutotileInternal)]
    struct Cool {}
}

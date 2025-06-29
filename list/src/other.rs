#[test]
fn debit_stack() {
    let mut data = 10;
    let ref1 = &mut data;
    unsafe {
        let ptr2 = ref1 as *mut i32;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;
        *ptr4 += 40;
        *ref3 += 30;
        *ptr2 += 20;
        *ref1 += 10;
    }
    println!("data: {data}");

    let mut data = [0, 0, 0];
    let ref1_at_0 = &mut data[0];
    let ptr2_at_0 = ref1_at_0 as *mut i32;
    let ptr3_at_0 = ptr2_at_0;
    unsafe {
        *ptr3_at_0 += 1;
        *ptr2_at_0 += 1;
        *ref1_at_0 += 1;
    }
    println!("data: {data:?}")
}

#[test]
fn debit_stack_arr() {
    // let mut data = [0; 10];
    // let ref1_at0 = &mut data[0];
    // let ref2_at1 = &mut data[1];
    // let ptr3_at0 = ref1_at0 as *mut i32;
    // let ptr4_at1 = ref2_at1 as *mut i32;

    // unsafe {
    //     *ptr4_at1 += 4;
    //     *ptr3_at0 += 3;
    //     *ref2_at1 += 2;
    //     *ref1_at0 += 1;
    // }

    // println!("data: {data:?}");
    let mut data = [0; 10];
    let slice1 = &mut data[..];
    let (slice2_at0, slice3_at1) = slice1.split_at_mut(1);
    let ref4_at0 = &mut slice2_at0[0];
    let ref5_at1 = &mut slice3_at1[0];
    let ptr6_at0 = ref4_at0 as *mut i32;
    let ptr7_at1 = ref5_at1 as *mut i32;
    unsafe {
        *ptr7_at1 += 7;
        *ptr6_at0 += 6;
        *ref5_at1 += 5;
        *ref4_at0 += 4;
    }
    println!("data: {data:?}")
}

#[test]
fn debit_stack_slisce() {
    let mut data = [0; 10];
    let slice1_all = &mut data[..];
    let ptr2_all = slice1_all.as_mut_ptr();
    unsafe {
        let ptr3_at0 = ptr2_all;
        let ptr4_at1 = ptr2_all.add(1);
        let ref5_at0 = &mut *ptr3_at0;
        let ref6_at1 = &mut *ptr4_at1;
        *ref6_at1 += 6;
        *ref5_at0 += 5;
        *ptr4_at1 += 4;
        *ptr3_at0 += 3;

        for i in 0..10 {
            *ptr2_all.add(i) += i;
        }
    }

    for (i, elem_ref) in slice1_all.iter_mut().enumerate() {
        *elem_ref += i;
    }

    println!("data: {data:?}");
}

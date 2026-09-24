fn main() {
    let mut v = vec![1,2,3] ;

    println!("Before: {:?}", v) ;   // Before: [1, 2, 3]

    let p_v = v.as_mut_ptr() ;  // получение *mut i32 из Vec<i32>

    unsafe {
        *p_v
            .add(    // добавляет беззнаковое смещение к указателю
            1   // Индекс через сырой указатель не проверяется на выход за границы.
            ) += 1 ;
    }

    println!("After: {:?}", v) ;    // After: [1, 3, 3]
}

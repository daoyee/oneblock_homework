
#[derive(PartialEq)]
pub enum SortDirection {
    ASC,//升序
    DES,//降序
}


pub fn bubble_sort<T: PartialOrd> (arr: &mut [T], sort_direction: SortDirection) {

    if arr.is_empty() { return; }

    let arr_len = arr.len();
    //比较的轮数为待排序元素总长度-1，即： arr.len() -1 轮
    // 每进行一轮比较交换，把最大（小）的元素移动到未排序数组的最后面
    for i in 0..arr_len - 1 {
        for j in 0..arr_len - 1 - i {

            if sort_direction == SortDirection::ASC {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            } else {
                if arr[j] < arr[j+1] {
                    arr.swap(j, j+1);
                }
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let mut vec1 = vec![6,5,3,200];
        bubble_sort(&mut vec1, SortDirection::ASC);
        assert_eq!(vec1,[3,5,6,200]);

        let mut vec2 = vec![6,5,3,200];
        bubble_sort(&mut vec2, SortDirection::DES);
        assert_eq!(vec2,[200,6,5,3]);
    }
}

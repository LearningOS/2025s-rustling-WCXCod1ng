/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn partition<T> (array: &mut [T], left: usize, right: usize) -> usize
    where T: PartialOrd + Copy
{
    // 现在考虑将[left, right]划分
    // 选择left作为p
    let p = left;
    let mut l = left;
    let mut r = right;
    let v = array[p];
    while l < r {
        // 先移动右侧
        while l < r && array[r] >= v {
            r -= 1;
        }
        // 覆盖到位置p
        array[p] = array[r];
        // 再移动左侧
        while l < r && array[l] <= v {
            l += 1;
        }
        // 覆盖到为止p
        array[p] = array[l];
    }
    // 返回最终的位置，l
    l
}

fn quick_sort<T>(array: &mut [T], left: usize, right: usize)
    where T: PartialOrd + Copy
{
    if left < right {
        let p = partition(array, left, right);
        quick_sort(array, left, p - 1);
        quick_sort(array, p+1, right);
    }
}

fn sort<T>(array: &mut [T])
    where T: PartialOrd + Copy
{
    quick_sort(array, 0, array.len()-1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
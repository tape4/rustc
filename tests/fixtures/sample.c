int add(int a, int b) {
    return a + b;
}

int factorial(int n) {
    int result = 1;
    for (int i = 1; i <= n; ++i) {
        result *= i;
    }
    return result;
}

char to_uppercase(char c) {
    // 'a'..'z' 범위이면 대문자로 변환
    if (c >= 'a' && c <= 'z') {
        return c - ('a' - 'A');
    }
    return c;
}

int main() {
    // 변수 초기화
    int x = 42;
    int oct = 007;       // 8진수 리터럴
    int *p = &x;         // 주소 연산자
    char ch1 = 'A';
    char ch2 = '\n';     // 이스케이프 시퀀스
    char ch3 = '\0';
    char buf[5] = {'h','e','l','l','o'};  // 고정 크기 배열
    int arr2[3];
    for (int i = 0; i < 3; i++) {
        arr2[i] = i * 2;
    }

    // 포인터 산술 및 복합 할당, 증감 연산
    *p += 10;
    x -= 5;
    x++;
    ++x;
    --x;
    x--;

    // 논리 연산자, 비교 연산자
    if (x > 10 && x < 100) {
        x = add(x, oct);
    } else if (x == 0 || x == -1) {
        x = factorial(5);
    }

    /* 다중 줄 주석입니다.
       비트 연산자 테스트 */
    int b_and = x & oct;
    int b_or  = x | oct;

    // while 루프와 continue/break
    int count = 0;
    while (count < 5) {
        if (count == 2) {
            count++;
            continue;
        }
        if (count == 4) {
            break;
        }
        count += 1;
    }

    return 0;
}
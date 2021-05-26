#define WASM_EXPORT __attribute__((visibility("default")))

int fib(int n)
{
    int a = 0, b = 1, c, i;
    if( n == 0)
        return a;
    for(i = 2; i <= n; i++)
    {
       c = a + b;
       a = b;
       b = c;
    }
    return b;
}

WASM_EXPORT
int main()
{
  int n = 9;
  return fib(n);
}
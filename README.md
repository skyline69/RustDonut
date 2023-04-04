# <p align="center"> __Rust Donut 🍩__ </p>
This is a Rust implementation of the famous **C Donut**. The code uses the Rayon library for parallel processing and is optimized for performance to make the donut rendering as fast as possible.
![ezgif com-optimize(2)](https://user-images.githubusercontent.com/67526259/229895567-f019ad52-b958-4373-8dc9-4f4c9d46e970.gif)

## Usage
To run the program do this:
<br><br>
![grafik](https://user-images.githubusercontent.com/67526259/229888088-b2ced43e-c0f1-4095-9ace-a465bf317f1a.png)


## Dependencies
- <a href="https://crates.io/crates/rayon">Rayon</a>

## License

This project is licensed under the MIT License.

## Credits

The original C implementation of the Donut was created by <a href="https://www.a1k0n.net/2011/07/20/donut-math.html">Andy Sloane</a>. This Rust implementation was created by skyline69.

## Notes

This implementation includes a trait UncheckedIndexExt to make the code safer when accessing elements of a string slice by index. This trait is not necessary, but improves the performance of the code.

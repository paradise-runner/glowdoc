# /// script
# dependencies = [
#   "Pillow"
# ]
# ///

import os
from PIL import Image

def create_diagonal_comparison_image(folder_path, output_filename="diagonal_comparison.png"):
    """
    Creates a diagonal comparison image from all images in a specified folder.

    The first image determines the final output dimensions. Subsequent images
    are resized to these dimensions and overlayed in diagonal sections
    in alphabetical order.

    Args:
        folder_path (str): The path to the folder containing the images.
        output_filename (str): The name of the output comparison image file.
    """
    image_files = []
    for filename in sorted(os.listdir(folder_path)):
        if filename.lower().endswith(('.png', '.jpg', '.jpeg', '.bmp', '.gif', '.tiff')):
            image_files.append(os.path.join(folder_path, filename))

    if not image_files:
        print(f"No supported image files found in '{folder_path}'.")
        return

    # Load the first image to determine the dimensions
    try:
        first_image = Image.open(image_files[0]).convert("RGBA")
        width, height = first_image.size
    except Exception as e:
        print(f"Error opening the first image '{image_files[0]}': {e}")
        return

    # Create a blank composite image with an alpha channel
    composite_image = Image.new("RGBA", (width, height), (0, 0, 0, 0))

    num_images = len(image_files)
    if num_images == 0:
        print("No images to process.")
        return

    # Calculate the step size for the diagonal sections
    # This determines how many pixels wide each diagonal "slice" will be for subsequent images
    # We are effectively dividing the diagonal length (or width/height for simplicity)
    # into num_images segments.
    # For a top-left to bottom-right diagonal, we can simplify this by iterating through width/height.
    step_size_x = width // num_images
    step_size_y = height // num_images

    print(f"Creating diagonal comparison image (Dimensions: {width}x{height}) from {num_images} images.")

    for i, img_path in enumerate(image_files):
        try:
            current_image = Image.open(img_path).convert("RGBA")
            current_image = current_image.resize((width, height), Image.Resampling.LANCZOS) # Use LANCZOS for quality

            # Calculate the coordinates for the diagonal slice
            # The diagonal goes from (0,0) to (width, height)
            # We're taking sections that are essentially "above" the diagonal line defined by
            # y = (height/width) * x, but for simplicity we'll use a series of triangles/trapezoids

            # The current approach will sequentially replace a larger bottom-left triangular region
            # This means the last image in the sequence will dominate the top-right.

            # Let's redefine the overlay logic for a more visually distinct "diagonal comparison"
            # We'll take a 'slice' from each image.
            # Imagine drawing lines parallel to the main diagonal.
            # Each image fills the region between two such parallel lines.

            # A more straightforward "diagonal reveal"
            # Image 0 covers the top-left triangle.
            # Image 1 covers the next diagonal strip.
            # ...
            # Image N covers the bottom-right triangle.

            # Simple "top-left to bottom-right" reveal logic:
            # We will progressively draw from top-left to bottom-right.
            # This is effective for conveying differences by gradually revealing parts of new images.

            # Create a mask for the current image's section
            mask = Image.new("L", (width, height), 0) # Black mask (transparent)

            # Draw a polygon for the current diagonal slice
            # The number of points depends on if we're doing triangular or trapezoidal sections.
            # Let's make it a sequence of trapezoids (or triangles at the ends).

            if num_images == 1:
                # If only one image, just use it as is
                composite_image.paste(current_image, (0, 0))
                break

            # Calculate the current section's start and end points along the diagonal
            # We are creating a sequence of trapezoids along the diagonal
            # The division points are on the main diagonal
            x_start_diag = int(i * (width / num_images))
            y_start_diag = int(i * (height / num_images))
            x_end_diag = int((i + 1) * (width / num_images))
            y_end_diag = int((i + 1) * (height / num_images))

            # Define the vertices for the trapezoidal slice for the current image
            # This approach makes sure each image contributes a distinct diagonal band.
            # The coordinates define a polygon that cuts through the image diagonally.
            # We essentially take the area between two parallel diagonals.

            # The polygon points depend on the direction of the diagonal.
            # For top-left to bottom-right, we want to reveal more of the new image
            # as we move towards the bottom right.

            # Let's simplify and make each image responsible for a triangular part
            # from the top-left, then progressively revealing more.
            # The first image is the full image, and subsequent images are "cut out"
            # starting from the top-left corner. This is a common "wipe" effect.

            # A better "diagonal comparison" is to have each image contribute a band.
            # For simplicity, we'll draw a diagonal line. The area *above and to the left*
            # of this line comes from one image, and *below and to the right* comes from another.
            # We'll then do this for N images, progressively taking more of the new image.

            # Let's try an overlay where the image fills the region from the top-left
            # up to a certain diagonal line.

            # The current image will overwrite the area from its current diagonal section onwards.
            # This creates a "sliding" reveal from top-left to bottom-right.

            # Define the corners for the polygon to create the "diagonal wipe" effect
            # We are effectively drawing a polygon that covers the area from (0,0) to
            # (x_end_diag, height) and (width, y_end_diag) to (width, height).
            # This will progressively reveal more of the current image.

            # A simpler approach for diagonal comparison with multiple images:
            # Each image replaces the previous one's content in a specific diagonal slice.
            # We can use a simple loop over the pixels.

            pixels_current = current_image.load()
            pixels_composite = composite_image.load()

            # The 'slice' for each image.
            # The 'i'th image will control the region where (x / width) + (y / height)
            # falls within a certain range defined by 'i'.

            # More simply, let's define a threshold for the diagonal.
            # The line is y = mx + c. For a top-left to bottom-right diagonal: y = (height/width) * x
            # We can shift this line for each image.

            # The "diagonal comparison" often implies a single image is split, but with multiple,
            # we need a clear methodology.
            # The most intuitive is a series of trapezoidal segments.

            # Let's define the diagonal based on the pixel index.
            # Each image gets a band. Band 'i' will be for pixels where
            # (x + y) / (width + height) is in a certain range.
            # This creates a consistent diagonal cut regardless of aspect ratio.

            # Normalize the position along the diagonal.
            # The 'normalized diagonal position' for a pixel (x,y) can be roughly considered as (x/width + y/height) / 2
            # Or, just x + y for a simple linear progression.
            # The total 'diagonal length' will be width + height.

            # The range for image 'i' will be from (i * (width + height) / num_images) to
            # ((i+1) * (width + height) / num_images).

            min_diag_sum = int(i * (width + height) / num_images)
            max_diag_sum = int((i + 1) * (width + height) / num_images)

            if i == num_images -1: # Last image covers everything from its start to the end
                max_diag_sum = width + height # Ensure the last image covers till the end

            for x in range(width):
                for y in range(height):
                    current_diag_sum = x + y # A simple representation of position along diagonal
                    if min_diag_sum <= current_diag_sum < max_diag_sum:
                        pixels_composite[x, y] = pixels_current[x, y]
                    elif i == 0 and current_diag_sum < min_diag_sum:
                        # For the very first image, fill the initial segment
                        pixels_composite[x, y] = pixels_current[x, y]

            # The above logic has an issue: only the last image would truly "stick" for its segment.
            # The previous ones would be overwritten.

            # Let's refine the logic to build up the image.
            # Start with the first image as the base.
            if i == 0:
                composite_image.paste(current_image, (0, 0))
            else:
                # For subsequent images, only paste the diagonal region.
                # Create a mask for the current image's diagonal slice.
                # This ensures previous parts are not overwritten where they shouldn't be.
                temp_mask = Image.new("L", (width, height), 0) # Black mask (transparent)
                pixels_mask = temp_mask.load()

                # Iterate through pixels to create the diagonal mask
                # The region to paste for image 'i' is where the "diagonal sum" (x+y)
                # is between min_diag_sum and max_diag_sum.
                for x_m in range(width):
                    for y_m in range(height):
                        current_diag_sum_m = x_m + y_m
                        if min_diag_sum <= current_diag_sum_m < max_diag_sum:
                            pixels_mask[x_m, y_m] = 255 # White (opaque)

                composite_image.paste(current_image, (0, 0), temp_mask)


        except Exception as e:
            print(f"Error processing image '{img_path}': {e}")
            continue

    try:
        composite_image.save(output_filename)
        print(f"Diagonal comparison image saved as '{output_filename}'")
    except Exception as e:
        print(f"Error saving the composite image: {e}")

# --- How to use the script ---
if __name__ == "__main__":
    # IMPORTANT: Replace 'path/to/your/image/folder' with the actual path
    # to the folder containing your images.
    image_folder = "screenshots"
    # Example usage:
    # image_folder = "C:/Users/YourUser/Pictures/MyComparisonImages"
    # image_folder = "/home/youruser/images/compare"

    # Make sure to change this to your actual folder path!
    if image_folder == "path/to/your/image/folder":
        print("Please update 'image_folder' variable in the script with the actual path to your image directory.")
    else:
        create_diagonal_comparison_image(image_folder)
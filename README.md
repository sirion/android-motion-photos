# Android Motion Photos Extractor

Utility to Extract Photo and Video from Android Motion Photos

Some android devices support "motion photos", popularized by apple as "live photos", basically small videos that go along with the main picture. On android devices these can for example be created by gcam (the google camera app). Not all gallery apps support these and if you bring your pictures onto your pc, there is (afaik) currently no easy way to access the video hidden in the image file.

This utitly simply extracts the image and the video, leaving the original file as it is and saves the image with the extension ".photo.jpg" and the video as ".video.mp4", keeping the rest of the filename as is.

The progress is printed to standard output and all files that are not image files containing a video are ignored.

## Usage

Extract from one picture file:

`android-motion-photos path/of/photos/my_image.jpg`

Extract all pictures in a folder:

`android-motion-photos path/of/photos/*`

Alternatively, you may just drag and drop files onto the executable, but you will not see the output if it is not done in a console/terminal.

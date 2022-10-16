from PIL import Image
from PIL import ImageDraw
from PIL import ImageFont
import sys
import os
import base64

padding_bottom = 30
font_dec_step = 2
font_size = 25

text = sys.argv[1]
bg_img = sys.argv[2]
font_str = sys.argv[3]
out_path = sys.argv[4]
img = Image.open(bg_img)
I1 = ImageDraw.Draw(img)
 
myFont = ImageFont.truetype(font_str, font_size)
sw,sh = myFont.getsize(text)

if sw > img.width * 0.8:
    while sw > img.width * 0.8:
        font_size -= font_dec_step
        myFont = ImageFont.truetype(font_str, font_size)
        sw,sh = myFont.getsize(text)

text_x = int((float(img.width) / 2.0) - (float(sw) / 2))
I1.text((text_x,img.height - padding_bottom - sh), text, fill=(255, 255, 255), embedded_color=True, font=myFont)
 
filename = base64.b64encode(text.encode("utf-8")).decode('utf-8')
path = os.path.join(out_path, "{}.png".format(filename))
img.save(path)
#img.save("{}.png".format(filename))

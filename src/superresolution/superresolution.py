import cv2
import numpy as np
import torch
import RRDBNet_arch as arch

# Paths
# All the paths are from root project directory so that the code runs
# with the run.sh script in the root directory
# If you wish to run this by itself, change the paths accordingly
model_path = 'src/superresolution/RRDB_ESRGAN_x4.pth'
device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
img_folder = 'images/'

# Load the model
model = arch.RRDBNet(3, 3, 64, 23, gc=32)
model.load_state_dict(torch.load(model_path), strict=True)
model.eval()
model = model.to(device)

print('Model path {:s}. \nTesting...'.format(model_path))

# Read the image
img = cv2.imread(img_folder + "out.png", cv2.IMREAD_COLOR)
img = img * 1.0 / 255
img = torch.from_numpy(np.transpose(img[:, :, [2, 1, 0]], (2, 0, 1))).float()
img_LR = img.unsqueeze(0)
img_LR = img_LR.to(device)

# Pass the image through the model
with torch.no_grad():
    output = model(img_LR).data.squeeze().float().cpu().clamp_(0, 1).numpy()

# Reformat the dimensions of the image
output = np.transpose(output[[2, 1, 0], :, :], (1, 2, 0))
output = (output * 255.0).round()

# Write the new image to the same folder
cv2.imwrite(img_folder + 'out_sp.png', output)

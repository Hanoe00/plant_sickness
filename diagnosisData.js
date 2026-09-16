// 1. Array index corresponds directly to model output index (0 to 37)
export const DISEASE_LABELS = [
  "Apple___Apple_scab",
  "Apple___Black_rot",
  "Apple___Cedar_apple_rust",
  "Apple___healthy",
  "Blueberry___healthy",
  "Cherry_(including_sour)___Powdery_mildew",
  "Cherry_(including_sour)___healthy",
  "Corn_(maize)___Cercospora_leaf_spot Gray_leaf_spot",
  "Corn_(maize)___Common_rust_",
  "Corn_(maize)___Northern_Leaf_Blight",
  "Corn_(maize)___healthy",
  "Grape___Black_rot",
  "Grape___Esca_(Black_Measles)",
  "Grape___Leaf_blight_(Isariopsis_Leaf_Spot)",
  "Grape___healthy",
  "Orange___Haunglongbing_(Citrus_greening)",
  "Peach___Bacterial_spot",
  "Peach___healthy",
  "Pepper,_bell___Bacterial_spot",
  "Pepper,_bell___healthy",
  "Potato___Early_blight",
  "Potato___Late_blight",
  "Potato___healthy",
  "Raspberry___healthy",
  "Soybean___healthy",
  "Squash___Powdery_mildew",
  "Strawberry___Leaf_scorch",
  "Strawberry___healthy",
  "Tomato___Bacterial_spot",
  "Tomato___Early_blight",
  "Tomato___Late_blight",
  "Tomato___Target_Spot",
  "Tomato___Tomato_Yellow_Leaf_Curl_Virus",
  "Tomato___Tomato_mosaic_virus",
  "Tomato___Two-spotted_spider_mite",
  "Tomato___Leaf_Mold",
  "Tomato___Septoria_leaf_spot",
  "Tomato___healthy"
];

// 2. Full Dictionary containing details, symptoms, and treatments
export const DIAGNOSIS_DATA = {
  // --- APPLE ---
  "Apple___Apple_scab": {
    name: "Apple Scab",
    plant: "Apple",
    status: "diseased",
    description: "Fungal infection caused by Venturia inaequalis leading to velvety dark spots on foliage and fruit.",
    symptoms: ["Olive-green to black velvety leaf spots", "Deformed or premature leaf drop", "Scabby patches on fruit"],
    treatment: ["Rake and destroy fallen leaves in autumn.", "Apply copper or sulfur fungicides at bud break.", "Prune for better airflow."]
  },
  "Apple___Black_rot": {
    name: "Black Rot",
    plant: "Apple",
    status: "diseased",
    description: "Fungal disease causing frog-eye leaf spots, bark cankers, and black mummified fruit.",
    symptoms: ["Frog-eye spots on leaves", "Shriveled black mummified fruit", "Cankers on branches"],
    treatment: ["Prune dead wood and cankers.", "Remove mummified fruit from trees and ground.", "Apply targeted fungicides post-bloom."]
  },
  "Apple___Cedar_apple_rust": {
    name: "Cedar Apple Rust",
    plant: "Apple",
    status: "diseased",
    description: "Fungal disease requiring junipers/cedars as alternating hosts.",
    symptoms: ["Bright orange/yellow spots on leaf upper surface", "Tube-like spore structures beneath leaves"],
    treatment: ["Remove wild junipers/cedars nearby.", "Apply preventative fungicide when pink buds appear."]
  },
  "Apple___healthy": {
    name: "Healthy Leaf",
    plant: "Apple",
    status: "healthy",
    description: "No detectable signs of fungal, bacterial, or pest damage.",
    symptoms: ["Smooth surface", "Uniform green color"],
    treatment: ["Maintain regular irrigation and balanced fertilization."]
  },

  // --- BLUEBERRY ---
  "Blueberry___healthy": {
    name: "Healthy Leaf",
    plant: "Blueberry",
    status: "healthy",
    description: "Healthy plant with vibrant green foliage.",
    symptoms: ["Vibrant green leaves", "No spots or discoloration"],
    treatment: ["Keep soil pH acidic (4.5–5.5) and water consistently."]
  },

  // --- CHERRY ---
  "Cherry_(including_sour)___Powdery_mildew": {
    name: "Powdery Mildew",
    plant: "Cherry",
    status: "diseased",
    description: "Fungal growth causing white dusty patches on younger leaves and shoot tips.",
    symptoms: ["Powdery white fungal patches on leaves", "Curling or distorted young leaves"],
    treatment: ["Apply sulfur or potassium bicarbonate sprays.", "Avoid overhead irrigation."]
  },
  "Cherry_(including_sour)___healthy": {
    name: "Healthy Leaf",
    plant: "Cherry",
    status: "healthy",
    description: "Healthy foliage without mildew or bacterial spot.",
    symptoms: ["Uniform texture and color"],
    treatment: ["Routine pruning and seasonal care."]
  },

  // --- CORN ---
  "Corn_(maize)___Cercospora_leaf_spot Gray_leaf_spot": {
    name: "Gray Leaf Spot",
    plant: "Corn",
    status: "diseased",
    description: "Fungal disease favored by warm, humid conditions causing rectangular leaf lesions.",
    symptoms: ["Rectangular tan or gray lesions restricted by leaf veins", "Blighting of foliage"],
    treatment: ["Rotate crops with non-grasses.", "Till crop residue under.", "Apply foliar fungicides if severe."]
  },
  "Corn_(maize)___Common_rust_": {
    name: "Common Rust",
    plant: "Corn",
    status: "diseased",
    description: "Fungal infection causing reddish-brown pustules on leaf surfaces.",
    symptoms: ["Golden-brown to cinnamon-brown pustules on both leaf sides"],
    treatment: ["Plant rust-resistant hybrids.", "Apply fungicides early if infection spreads."]
  },
  "Corn_(maize)___Northern_Leaf_Blight": {
    name: "Northern Leaf Blight",
    plant: "Corn",
    status: "diseased",
    description: "Fungal pathogen producing cigar-shaped grayish lesions.",
    symptoms: ["Long, elliptical (cigar-shaped) grayish-green or tan lesions"],
    treatment: ["Use resistant corn hybrids.", "Practice crop rotation and field tillage."]
  },
  "Corn_(maize)___healthy": {
    name: "Healthy Leaf",
    plant: "Corn",
    status: "healthy",
    description: "Healthy stalk and leaf development.",
    symptoms: ["Strong green color", "Unblemished leaf blades"],
    treatment: ["Maintain adequate nitrogen and irrigation levels."]
  },

  // --- GRAPE ---
  "Grape___Black_rot": {
    name: "Black Rot",
    plant: "Grape",
    status: "diseased",
    description: "Severe fungal disease attacking leaves, shoots, and berries.",
    symptoms: ["Small reddish-brown leaf spots", "Berries turning brown, shriveling into hard black mummies"],
    treatment: ["Destroy mummified grapes.", "Apply fungicides starting early in the bloom cycle."]
  },
  "Grape___Esca_(Black_Measles)": {
    name: "Esca (Black Measles)",
    plant: "Grape",
    status: "diseased",
    description: "Complex trunk disease causing tiger-stripe patterns on foliage.",
    symptoms: ["'Tiger-stripe' interveinal chlorosis/necrosis", "Dark specks on berries"],
    treatment: ["Prune affected vine sections.", "Protect pruning wounds with sealants."]
  },
  "Grape___Leaf_blight_(Isariopsis_Leaf_Spot)": {
    name: "Leaf Blight",
    plant: "Grape",
    status: "diseased",
    description: "Fungal spot disease causing necrotic blotches on leaf margins.",
    symptoms: ["Irregular brown spots with dark borders", "Premature defoliation"],
    treatment: ["Spray copper-based fungicides.", "Rake up and burn infected autumn leaves."]
  },
  "Grape___healthy": {
    name: "Healthy Leaf",
    plant: "Grape",
    status: "healthy",
    description: "Healthy grape canopy.",
    symptoms: ["Clear green foliage without lesions"],
    treatment: ["Proper trellising and canopy management."]
  },

  // --- ORANGE ---
  "Orange___Haunglongbing_(Citrus_greening)": {
    name: "Citrus Greening (HLB)",
    plant: "Orange",
    status: "diseased",
    description: "Bacterial disease spread by psyllids causing asymmetric yellowing and bitter fruit.",
    symptoms: ["Asymmetric yellow mottling on leaves", "Small, lopsided fruit that stays green"],
    treatment: ["Control psyllid insect vectors.", "Remove infected trees immediately."]
  },

  // --- PEACH ---
  "Peach___Bacterial_spot": {
    name: "Bacterial Spot",
    plant: "Peach",
    status: "diseased",
    description: "Bacterial pathogen causing shot-hole symptoms on leaves and pitted fruit.",
    symptoms: ["Small dark spots dropping out to leave 'shot holes'", "Pitted fruit"],
    treatment: ["Apply copper sprays during dormancy.", "Plant resistant peach cultivars."]
  },
  "Peach___healthy": {
    name: "Healthy Leaf",
    plant: "Peach",
    status: "healthy",
    description: "Healthy peach leaf structure.",
    symptoms: ["Clear green leaves", "No holes or spotting"],
    treatment: ["Prune annually for sunlight penetration."]
  },

  // --- PEPPER ---
  "Pepper,_bell___Bacterial_spot": {
    name: "Bacterial Spot",
    plant: "Bell Pepper",
    status: "diseased",
    description: "Bacterial leaf and fruit spot widespread in warm, wet weather.",
    symptoms: ["Small water-soaked leaf spots turning dark brown", "Scabby raised spots on fruit"],
    treatment: ["Use copper-based sprays mixed with mancozeb.", "Avoid overhead watering."]
  },
  "Pepper,_bell___healthy": {
    name: "Healthy Leaf",
    plant: "Bell Pepper",
    status: "healthy",
    description: "Healthy pepper foliage.",
    symptoms: ["Smooth green leaves"],
    treatment: ["Maintain consistent watering."]
  },

  // --- POTATO ---
  "Potato___Early_blight": {
    name: "Early Blight",
    plant: "Potato",
    status: "diseased",
    description: "Fungal disease causing target-like concentric rings on older leaves.",
    symptoms: ["Brown spots with concentric 'target' rings", "Yellowing around spots"],
    treatment: ["Apply bio-fungicides or copper sprays.", "Maintain proper plant spacing."]
  },
  "Potato___Late_blight": {
    name: "Late Blight",
    plant: "Potato",
    status: "diseased",
    description: "Aggressive water mold (Phytophthora infestans) that can rapidly destroy plants.",
    symptoms: ["Large dark water-soaked lesions", "White fungal mold under leaves in wet conditions"],
    treatment: ["Apply preventative copper/mancozeb fungicides.", "Destroy infected vines promptly."]
  },
  "Potato___healthy": {
    name: "Healthy Leaf",
    plant: "Potato",
    status: "healthy",
    description: "Healthy potato plant foliage.",
    symptoms: ["Vigorous green leaves"],
    treatment: ["Hill soil properly around plants."]
  },

  // --- RASPBERRY & SOYBEAN ---
  "Raspberry___healthy": {
    name: "Healthy Leaf",
    plant: "Raspberry",
    status: "healthy",
    description: "Healthy raspberry cane foliage.",
    symptoms: ["Unblemished leaves"],
    treatment: ["Prune spent canes annually."]
  },
  "Soybean___healthy": {
    name: "Healthy Leaf",
    plant: "Soybean",
    status: "healthy",
    description: "Healthy soybean canopy.",
    symptoms: ["Trifoliate leaves with vibrant green color"],
    treatment: ["Standard crop management."]
  },

  // --- SQUASH & STRAWBERRY ---
  "Squash___Powdery_mildew": {
    name: "Powdery Mildew",
    plant: "Squash",
    status: "diseased",
    description: "White powdery fungal growth spreading over leaf surfaces.",
    symptoms: ["Dusty white spots covering upper/lower leaf surfaces"],
    treatment: ["Spray neem oil or potassium bicarbonate.", "Improve air circulation."]
  },
  "Strawberry___Leaf_scorch": {
    name: "Leaf Scorch",
    plant: "Strawberry",
    status: "diseased",
    description: "Fungal spot disease leading to leaf drying and scorching appearance.",
    symptoms: ["Dark purple to red spots blending together", "Leaves looking dried/burnt"],
    treatment: ["Remove infected foliage post-harvest.", "Apply fungicides in early spring."]
  },
  "Strawberry___healthy": {
    name: "Healthy Leaf",
    plant: "Strawberry",
    status: "healthy",
    description: "Healthy strawberry plant.",
    symptoms: ["Deep green trifoliate leaves"],
    treatment: ["Keep soil mulched with straw."]
  },

  // --- TOMATO ---
  "Tomato___Bacterial_spot": {
    name: "Bacterial Spot",
    plant: "Tomato",
    status: "diseased",
    description: "Bacterial infection causing leaf speckling and fruit spots.",
    symptoms: ["Small black/dark brown spots with yellow halos", "Raised spots on green fruit"],
    treatment: ["Spray copper hydroxide.", "Avoid touching plants when foliage is wet."]
  },
  "Tomato___Early_blight": {
    name: "Early Blight",
    plant: "Tomato",
    status: "diseased",
    description: "Target-ring spots starting on lower, older leaves.",
    symptoms: ["Concentric ring brown spots on lower leaves", "Yellowing around spots"],
    treatment: ["Mulch under plants to prevent soil splash.", "Apply copper fungicides."]
  },
  "Tomato___Late_blight": {
    name: "Late Blight",
    plant: "Tomato",
    status: "diseased",
    description: "Destructive mold spreading fast in cool, wet weather.",
    symptoms: ["Greasy grayish-brown spots", "White fuzzy mold under leaves"],
    treatment: ["Remove infected branches immediately.", "Spray copper fungicides preventatively."]
  },
  "Tomato___Leaf_Mold": {
    name: "Leaf Mold",
    plant: "Tomato",
    status: "diseased",
    description: "Fungal issue common in high-humidity environments/greenhouses.",
    symptoms: ["Pale yellow spots on upper leaf side", "Olive-green velvet mold underneath"],
    treatment: ["Increase greenhouse ventilation.", "Apply fungicide if humidity stays high."]
  },
  "Tomato___Septoria_leaf_spot": {
    name: "Septoria Leaf Spot",
    plant: "Tomato",
    status: "diseased",
    description: "Fungal spots with dark borders and tan centers containing specks.",
    symptoms: ["Circular spots with gray/tan centers and dark borders"],
    treatment: ["Remove affected bottom leaves.", "Avoid overhead irrigation."]
  },
  "Tomato___Target_Spot": {
    name: "Target Spot",
    plant: "Tomato",
    status: "diseased",
    description: "Fungal pathogen causing target-like lesions on foliage.",
    symptoms: ["Necrotic spots with light brown centers and dark rings"],
    treatment: ["Improve row spacing.", "Apply preventative fungicide."]
  },
  "Tomato___Two-spotted_spider_mite": {
    name: "Spider Mites",
    plant: "Tomato",
    status: "diseased",
    description: "Tiny pests feeding on leaf sap, leaving speckled webbing.",
    symptoms: ["Yellow speckled stippling on leaves", "Fine webbing on leaf undersides"],
    treatment: ["Spray insecticidal soap or neem oil.", "Increase humidity around foliage."]
  },
  "Tomato___Tomato_Yellow_Leaf_Curl_Virus": {
    name: "Yellow Leaf Curl Virus (TYLCV)",
    plant: "Tomato",
    status: "diseased",
    description: "Viral infection transmitted by whiteflies.",
    symptoms: ["Leaves curling upward and yellowing", "Stunted plant growth"],
    treatment: ["Control whiteflies with yellow sticky traps or nets.", "Remove infected plants."]
  },
  "Tomato___Tomato_mosaic_virus": {
    name: "Tomato Mosaic Virus (ToMV)",
    plant: "Tomato",
    status: "diseased",
    description: "Highly contagious virus causing mottling and leaf distortion.",
    symptoms: ["Mottled light and dark green patterns on leaves", "Fern-like leaf distortion"],
    treatment: ["Disinfect tools with bleach solution.", "Remove and destroy infected plants."]
  },
  "Tomato___healthy": {
    name: "Healthy Leaf",
    plant: "Tomato",
    status: "healthy",
    description: "Healthy tomato plant foliage.",
    symptoms: ["Vibrant green leaves", "No spots, curling, or pests"],
    treatment: ["Prune suckers and stake vines properly."]
  }
};
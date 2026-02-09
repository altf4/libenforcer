import init, { analyze_replay } from './pkg/libenforcer_wasm.js';

let wasmInitialized = false;
let selectedFile = null;

// Initialize when page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Set up event listeners
    const fileInput = document.getElementById('slpFile');
    const analyzeBtn = document.getElementById('analyzeBtn');

    fileInput.addEventListener('change', handleFileSelect);
    analyzeBtn.addEventListener('click', analyzeFile);
});

function handleFileSelect(event) {
    const file = event.target.files[0];
    if (file) {
        selectedFile = file;
        document.getElementById('analyzeBtn').disabled = false;

        // Update label text
        const fileLabel = document.querySelector('.file-text');
        fileLabel.textContent = `Selected: ${file.name}`;
    }
}

async function analyzeFile() {
    if (!selectedFile) {
        alert('Please select a .SLP file first');
        return;
    }

    // Show loading, hide results
    document.getElementById('loadingSection').style.display = 'block';
    document.getElementById('resultsSection').style.display = 'none';

    try {
        // Initialize WASM module if needed
        if (!wasmInitialized) {
            await init();
            wasmInitialized = true;
            console.log('WASM module initialized');
        }

        // Read file as ArrayBuffer
        const arrayBuffer = await selectedFile.arrayBuffer();
        const bytes = new Uint8Array(arrayBuffer);

        // Analyze each player (0-3)
        const allResults = {};
        for (let playerIndex = 0; playerIndex < 4; playerIndex++) {
            try {
                const result = analyze_replay(bytes, playerIndex);
                allResults[playerIndex] = result;
            } catch (e) {
                // Player doesn't exist in this game
                console.log(`Player ${playerIndex} not found:`, e);
            }
        }

        // Display results
        displayResults(allResults);

    } catch (error) {
        console.error('Error analyzing file:', error);
        alert(`Error: ${error.message || error}`);
    } finally {
        document.getElementById('loadingSection').style.display = 'none';
    }
}

function displayResults(allResults) {
    const resultsDiv = document.getElementById('results');
    const resultsSection = document.getElementById('resultsSection');

    resultsDiv.innerHTML = '';

    if (Object.keys(allResults).length === 0) {
        resultsDiv.innerHTML = '<p>No players found in this replay.</p>';
        resultsSection.style.display = 'block';
        return;
    }

    for (const [playerIndex, results] of Object.entries(allResults)) {
        const playerDiv = document.createElement('div');
        playerDiv.className = 'player-results';

        const playerHeader = document.createElement('h3');
        playerHeader.textContent = `Port ${parseInt(playerIndex) + 1}`;
        playerDiv.appendChild(playerHeader);

        // Check names mapping
        const checkNames = {
            travel_time: 'Box Travel Time',
            disallowed_cstick: 'Disallowed C-Stick Values',
            uptilt_rounding: 'Uptilt Rounding',
            crouch_uptilt: 'Fast Crouch Uptilt',
            sdi: 'Illegal SDI',
            goomwave: 'GoomWave Clamping'
        };

        // Display each check result
        for (const [checkKey, checkName] of Object.entries(checkNames)) {
            const checkResult = results[checkKey];
            if (!checkResult) continue;

            const resultDiv = document.createElement('div');
            resultDiv.className = `check-result ${checkResult.result ? 'fail' : 'pass'}`;

            const nameSpan = document.createElement('span');
            nameSpan.className = 'check-name';
            nameSpan.textContent = checkName;

            const statusSpan = document.createElement('span');
            statusSpan.className = 'check-status';

            if (checkResult.result) {
                statusSpan.textContent = '❌ VIOLATION';
                if (checkResult.violations && checkResult.violations.length > 0) {
                    const countSpan = document.createElement('span');
                    countSpan.className = 'violation-count';
                    countSpan.textContent = `(${checkResult.violations.length} violation${checkResult.violations.length > 1 ? 's' : ''})`;
                    statusSpan.appendChild(countSpan);
                }
            } else {
                statusSpan.textContent = '✅ PASS';
            }

            resultDiv.appendChild(nameSpan);
            resultDiv.appendChild(statusSpan);
            playerDiv.appendChild(resultDiv);
        }

        resultsDiv.appendChild(playerDiv);
    }

    resultsSection.style.display = 'block';
}

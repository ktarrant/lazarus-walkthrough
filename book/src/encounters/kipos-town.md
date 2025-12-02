<!-- area-id: kipos-town -->
### Kipos Town

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=beautifly">Beautifly</a> | 2% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="beautifly" /> |
| <a href="../pokemon-lookup.html?q=doduo">Doduo</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="doduo" /> |
| <a href="../pokemon-lookup.html?q=eevee">Eevee</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="eevee" /> |
| <a href="../pokemon-lookup.html?q=igglybuff">Igglybuff</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="igglybuff" /> |
| <a href="../pokemon-lookup.html?q=magnemite">Magnemite</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="magnemite" /> |
| <a href="../pokemon-lookup.html?q=pikachu">Pikachu</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pikachu" /> |
| <a href="../pokemon-lookup.html?q=stantler">Stantler</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="stantler" /> |
| <a href="../pokemon-lookup.html?q=steenee">Steenee</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="steenee" /> |
| <a href="../pokemon-lookup.html?q=timburr">Timburr</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="timburr" /> |
| <a href="../pokemon-lookup.html?q=dustox">Dustox</a> | — | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dustox" /> |
| <a href="../pokemon-lookup.html?q=pumpkaboo">Pumpkaboo</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pumpkaboo" /> |
| <a href="../pokemon-lookup.html?q=stunky">Stunky</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="stunky" /> |
| <a href="../pokemon-lookup.html?q=corsola">Corsola</a> | — | — | 5% | — | — | 15% | <input type="checkbox" class="caught-check" data-species="corsola" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 30% | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=galarian-corsola">Galarian Corsola</a> | — | — | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="galarian-corsola" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="skrelp" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |

<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>


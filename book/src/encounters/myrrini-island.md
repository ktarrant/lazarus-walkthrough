<!-- area-id: myrrini-island -->
### Myrrini Island

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=cherubi">Cherubi</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cherubi" /> |
| <a href="../pokemon-lookup.html?q=hawlucha">Hawlucha</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hawlucha" /> |
| <a href="../pokemon-lookup.html?q=jigglypuff">Jigglypuff</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="jigglypuff" /> |
| <a href="../pokemon-lookup.html?q=meditite">Meditite</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="meditite" /> |
| <a href="../pokemon-lookup.html?q=oricorio-baile">Oricorio Baile</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="oricorio-baile" /> |
| <a href="../pokemon-lookup.html?q=oricorio-pau">Oricorio Pa'u</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="oricorio-pau" /> |
| <a href="../pokemon-lookup.html?q=oricorio-pom-pom">Oricorio Pom-Pom</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="oricorio-pom-pom" /> |
| <a href="../pokemon-lookup.html?q=oricorio-sensu">Oricorio Sensu</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="oricorio-sensu" /> |
| <a href="../pokemon-lookup.html?q=pancham">Pancham</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pancham" /> |
| <a href="../pokemon-lookup.html?q=steenee">Steenee</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="steenee" /> |
| <a href="../pokemon-lookup.html?q=drowzee">Drowzee</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="drowzee" /> |
| <a href="../pokemon-lookup.html?q=floette-blue-flower">Floette Blue Flower</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="floette-blue-flower" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | 10% | — | — | — | <input type="checkbox" class="caught-check" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=unovan-basculin">Unovan Basculin</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="unovan-basculin" /> |
| <a href="../pokemon-lookup.html?q=white-striped-basculin">White-Striped Basculin</a> | — | — | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="white-striped-basculin" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | — | 30% | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="caught-check" data-species="skrelp" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | <input type="checkbox" class="caught-check" data-species="chinchou" /> |
| <a href="../pokemon-lookup.html?q=lanturn">Lanturn</a> | — | — | — | — | — | 5% | <input type="checkbox" class="caught-check" data-species="lanturn" /> |
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


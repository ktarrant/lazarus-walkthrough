<!-- area-id: sea-of-asteri-east -->
### Sea of Asteri (East)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | Dive | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=charcadet">Charcadet</a> | 5% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="charcadet" /> |
| <a href="../pokemon-lookup.html?q=charjabug">Charjabug</a> | 10% | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="charjabug" /> |
| <a href="../pokemon-lookup.html?q=crabrawler">Crabrawler</a> | 20% | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="crabrawler" /> |
| <a href="../pokemon-lookup.html?q=cufant">Cufant</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cufant" /> |
| <a href="../pokemon-lookup.html?q=jangmo-o">Jangmo-o</a> | 5% | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="jangmo-o" /> |
| <a href="../pokemon-lookup.html?q=ninjask">Ninjask</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="ninjask" /> |
| <a href="../pokemon-lookup.html?q=rufflet">Rufflet</a> | 20% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="rufflet" /> |
| <a href="../pokemon-lookup.html?q=stufful">Stufful</a> | 10% | — | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="stufful" /> |
| <a href="../pokemon-lookup.html?q=trumbeak">Trumbeak</a> | 10% | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="trumbeak" /> |
| <a href="../pokemon-lookup.html?q=chespin">Chespin</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="chespin" /> |
| <a href="../pokemon-lookup.html?q=fennekin">Fennekin</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="fennekin" /> |
| <a href="../pokemon-lookup.html?q=froakie">Froakie</a> | — | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="froakie" /> |
| <a href="../pokemon-lookup.html?q=shedinja">Shedinja</a> | — | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="shedinja" /> |
| <a href="../pokemon-lookup.html?q=togepi">Togepi</a> | — | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="togepi" /> |
| <a href="../pokemon-lookup.html?q=vullaby">Vullaby</a> | — | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="vullaby" /> |
| <a href="../pokemon-lookup.html?q=skrelp">Skrelp</a> | — | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="skrelp" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | 20% | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wattrel">Wattrel</a> | — | — | 30% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="wattrel" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | — | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | 20% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | — | <input type="checkbox" class="caught-check" data-species="chinchou" /> |
| <a href="../pokemon-lookup.html?q=lanturn">Lanturn</a> | — | — | — | — | — | 5% | — | <input type="checkbox" class="caught-check" data-species="lanturn" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | 20% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=corphish">Corphish</a> | — | — | — | — | — | — | 10% | <input type="checkbox" class="caught-check" data-species="corphish" /> |

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


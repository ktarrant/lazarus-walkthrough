<!-- area-id: erinys-path-east -->
### Erinys Path (East)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=cherubi">Cherubi</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="cherubi" /> |
| <a href="../pokemon-lookup.html?q=falinks">Falinks</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="falinks" /> |
| <a href="../pokemon-lookup.html?q=gastly">Gastly</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gastly" /> |
| <a href="../pokemon-lookup.html?q=houndour">Houndour</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="houndour" /> |
| <a href="../pokemon-lookup.html?q=kabuto">Kabuto</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="kabuto" /> |
| <a href="../pokemon-lookup.html?q=mimikyu">Mimikyu</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="mimikyu" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Green Core</a> | 4% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Yellow Core</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=onix">Onix</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="onix" /> |
| <a href="../pokemon-lookup.html?q=spinarak">Spinarak</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="spinarak" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=minior-core">Minior Violet Core</a> | — | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="minior-core" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=skorupi">Skorupi</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="skorupi" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | 10% | — | — | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | — | — | 60% | — | 20% | — | <input type="checkbox" class="caught-check" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=corphish">Corphish</a> | — | — | — | — | — | 20% | <input type="checkbox" class="caught-check" data-species="corphish" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="remoraid" /> |

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


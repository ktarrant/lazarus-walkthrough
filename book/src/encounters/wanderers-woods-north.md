<!-- area-id: wanderers-woods-north -->
### Wanderer's Woods (North)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aipom">Aipom</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="aipom" /> |
| <a href="../pokemon-lookup.html?q=applin">Applin</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="applin" /> |
| <a href="../pokemon-lookup.html?q=blitzle">Blitzle</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="blitzle" /> |
| <a href="../pokemon-lookup.html?q=chikorita">Chikorita</a> | 4% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="chikorita" /> |
| <a href="../pokemon-lookup.html?q=heracross">Heracross</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="heracross" /> |
| <a href="../pokemon-lookup.html?q=pichu">Pichu</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="pichu" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | 5% | 5% | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=pumpkaboo">Pumpkaboo</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="pumpkaboo" /> |
| <a href="../pokemon-lookup.html?q=shroomish">Shroomish</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="shroomish" /> |
| <a href="../pokemon-lookup.html?q=toedscool">Toedscool</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="toedscool" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | 10% | — | — | — | 40% | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="wooper" /> |
| <a href="../pokemon-lookup.html?q=impidimp">Impidimp</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="impidimp" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="paldean-wooper" /> |
| <a href="../pokemon-lookup.html?q=shuppet">Shuppet</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="shuppet" /> |
| <a href="../pokemon-lookup.html?q=togedemaru">Togedemaru</a> | — | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="togedemaru" /> |
| <a href="../pokemon-lookup.html?q=ducklett">Ducklett</a> | — | — | 10% | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="ducklett" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | — | — | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=barboach">Barboach</a> | — | — | — | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="barboach" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="wanderers-woods-north" data-species="remoraid" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

